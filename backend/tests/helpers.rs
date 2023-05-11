use actix_web::web::Json;
use diesel::r2d2::ConnectionManager;
use diesel::{pg::Pg, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use pace::handlers::theme::ThemeData;
use r2d2::{Pool, PooledConnection};
use reqwest::{Client, Response};
use serde_json::Map;
use std::error::Error;
use std::io::{Cursor, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use testcontainers::clients::Cli;
use testcontainers::images::postgres::Postgres;
use testcontainers::Container;

use pace::core::service::EmailConfiguration;
use pace::models::info::Info;
use pace::models::users::LoginData;
use pace::{get_connection_pool, run};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

mockall::lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let templates = match tera::Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        templates
    };

    static ref CERTIFICATE_PATHS: (&'static str, &'static str) = {
        use rcgen::generate_simple_self_signed;

        let subject_alt_names = vec!["localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
        let pem_serialized = cert.serialize_pem().unwrap();
        std::fs::create_dir_all("tests/.cert/").unwrap();
        let cert_path = "tests/.cert/cert.pem";
        std::fs::write(cert_path, &pem_serialized.as_bytes()).unwrap();
        let key_path = "tests/.cert/key.pem"; // talisman-ignore-line; this is a self-signed cert key used for testing only
        std::fs::write(key_path, &cert.serialize_private_key_pem().as_bytes()).unwrap(); // talisman-ignore-line; this is a self-signed cert key used for testing only

        (cert_path, key_path)
    };
}

#[derive(Clone)]
struct TestHandler {
    sender_mails: Arc<Mutex<Vec<String>>>,
    recipient_mails: Arc<Mutex<Vec<Vec<String>>>>,
    cursor: Arc<Mutex<Cursor<Vec<u8>>>>,
}

impl TestHandler {
    fn new() -> Self {
        Self {
            sender_mails: Arc::new(Mutex::new(vec![])),
            recipient_mails: Arc::new(Mutex::new(vec![])),
            cursor: Arc::new(Mutex::new(Cursor::new(vec![]))),
        }
    }
}

impl mailin_embedded::Handler for TestHandler {
    fn helo(
        &mut self,
        _ip: std::net::IpAddr,
        _domain: &str,
    ) -> mailin_embedded::response::Response {
        mailin_embedded::response::OK
    }

    fn mail(
        &mut self,
        _ip: std::net::IpAddr,
        _domain: &str,
        from: &str,
    ) -> mailin_embedded::response::Response {
        dbg!(from);
        self.sender_mails.lock().unwrap().push(from.to_string());
        mailin_embedded::response::OK
    }

    fn data_start(
        &mut self,
        _domain: &str,
        _from: &str,
        _is8bit: bool,
        to: &[String],
    ) -> mailin_embedded::Response {
        self.recipient_mails
            .lock()
            .unwrap()
            .push(to.iter().map(|v| v.to_string()).collect());

        mailin_embedded::response::OK
    }

    fn data(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.cursor.lock().unwrap().write(buf).map(|_| ())
    }

    fn auth_plain(
        &mut self,
        _authorization_id: &str,
        _authentication_id: &str,
        _password: &str,
    ) -> mailin_embedded::response::Response {
        mailin_embedded::response::AUTH_OK
    }
}

pub struct TestEmailServer {
    email_configuration: EmailConfiguration,
    handler: TestHandler,
    pub server_join_handle: JoinHandle<()>,
}

impl TestEmailServer {
    pub fn new(number_of_threads: Option<u32>) -> anyhow::Result<Self> {
        let handler = TestHandler::new();

        let handler_clone = handler.clone();
        let socket = TcpListener::bind("localhost:0").unwrap();
        let addr = socket.local_addr().unwrap();
        let server_join_handle = std::thread::spawn(move || {
            let mut server = mailin_embedded::Server::new(handler_clone);

            server
                .with_name("localhost")
                .with_auth(mailin_embedded::AuthMechanism::Plain)
                .with_ssl(mailin_embedded::SslConfig::SelfSigned {
                    cert_path: CERTIFICATE_PATHS.0.to_string(),
                    key_path: CERTIFICATE_PATHS.1.to_string(), // talisman-ignore-line; this is a self-signed cert key used for testing only
                })
                .unwrap()
                // we need to use more threads than the actix_web application to prevent blocking
                .with_num_threads(
                    number_of_threads.unwrap_or_else(|| (num_cpus::get_physical() + 1) as u32),
                )
                .with_tcp_listener(socket);
            server.serve().unwrap();
        });

        let email_configuration = EmailConfiguration::new(
            "email@example.com".to_string(),
            Some("Display Name".to_string()),
            "localhost".to_string(),
            Some(addr.port()),
            "foo".to_string(),
            "bar".to_string(),
            Some(CERTIFICATE_PATHS.0.to_string()),
        );

        Ok(Self {
            email_configuration,
            handler,
            server_join_handle,
        })
    }

    pub fn get_configuration(&self) -> EmailConfiguration {
        self.email_configuration.clone()
    }

    pub fn get_last_sender_email_address(&self) -> Option<String> {
        self.handler
            .sender_mails
            .lock()
            .unwrap()
            .last()
            .map(|v| v.to_string())
    }

    pub fn get_last_recipient_email_addresses(&self) -> Vec<String> {
        self.handler
            .recipient_mails
            .lock()
            .unwrap()
            .last()
            .map(|v| v.clone())
            .unwrap_or_else(|| Vec::new())
    }

    pub fn get_last_mail_data(&self) -> Option<String> {
        Some(
            std::str::from_utf8(self.handler.cursor.lock().unwrap().get_ref())
                .map(|v| v.to_string())
                .unwrap(),
        )
    }
}

pub struct TestDatabase<'a> {
    _database: Container<'a, Postgres>,
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl<'a> TestDatabase<'a> {
    fn start_up(
        docker: &'a Cli,
    ) -> (
        Container<'a, Postgres>,
        Pool<ConnectionManager<PgConnection>>,
    ) {
        let image: Postgres = Postgres::default();
        let pg_db = "postgres";
        let pg_user = "postgres";
        let pg_password = "postgres"; // talisman-ignore-line
        let database: Container<'a, Postgres> = docker.run(image);
        let pg_port = database.get_host_port_ipv4(5432);
        let database_connection_string =
            format!("postgres://{pg_user}:{pg_password}@127.0.0.1:{pg_port}/{pg_db}"); // talisman-ignore-line
        let connection_pool = get_connection_pool(Some(database_connection_string))
            .expect("Could not initialize connection pool");
        (database, connection_pool)
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Pg>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        connection.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }

    pub fn with_migrations(docker: &'a Cli) -> Self {
        let (database, connection_pool): (
            Container<'a, Postgres>,
            Pool<ConnectionManager<PgConnection>>,
        ) = Self::start_up(docker);
        let mut connection = connection_pool.get().expect("Unable to get connection");
        Self::run_migrations(&mut connection).expect("Unable to run migrations");
        Self {
            _database: database,
            connection_pool,
        }
    }

    pub fn without_migrations(docker: &'a Cli) -> Self {
        let (database, connection_pool) = Self::start_up(docker);

        Self {
            _database: database,
            connection_pool,
        }
    }

    pub fn get_connection_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.connection_pool.clone()
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.connection_pool
            .get()
            .expect("Unable to get connection")
    }
}

pub struct TestApp<'a> {
    database: TestDatabase<'a>,
    client: Client,
    address: String,
    email_server: Option<TestEmailServer>,
}

impl<'a> TestApp<'a> {
    pub async fn new(docker: &'a Cli) -> TestApp<'a> {
        let database = TestDatabase::with_migrations(docker);

        let email_server = TestEmailServer::new(None).expect("Unable to start test email server");

        let client = Client::new();
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("Unable to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        let server = run(
            listener,
            database.connection_pool.clone(),
            Some(email_server.email_configuration.clone()),
        )
        .expect("Unable to bind address");

        let _ = tokio::spawn(server);

        Self {
            database,
            client,
            address,
            email_server: Some(email_server),
        }
    }

    pub async fn create_app(&self) -> String {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("Unable to bind random port");
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, self.database.connection_pool.clone(), None)
            .expect("Unable to bind address");

        let _ = tokio::spawn(server);

        format!("http://127.0.0.1:{}", port)
    }

    pub async fn get_admin_cookie(&self) -> String {
        let login_response = self
            .client
            .post(format!("{}/api/admin/login", self.address))
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(&LoginData {
                    username: "admin".to_string(),
                    password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
                })
                .unwrap(),
            )
            .send()
            .await
            .expect("Unable to send request.");

        let cookie = login_response
            .cookies()
            .next()
            .expect("Unable to get cookie");

        format!("{}={}", cookie.name(), cookie.value())
    }

    pub async fn create_runner(&self, participant: Info) -> Response {
        self.client
            .post(format!("{}/api/runners", self.address))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&Json(participant)).unwrap())
            .send()
            .await
            .expect("Unable to send request.")
    }

    pub async fn update_theme(&self, theme_data: ThemeData, cookie: Option<String>) -> Response {
        let mut request_builder = self
            .client
            .put(format!("{}/api/theme", self.address))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&Json(theme_data)).unwrap());

        if cookie.is_some() {
            request_builder = request_builder.header("Cookie", cookie.unwrap());
        }

        request_builder
            .send()
            .await
            .expect("Unable to send request.")
    }

    pub async fn delete_rejected_transactions(
        &self,
        body: String,
        cookie: Option<String>,
    ) -> Response {
        let mut request_builder = self
            .client
            .put(format!("{}/api/admin/finance/delete", self.address))
            .header("Content-Type", "application/json")
            .body(body);

        if cookie.is_some() {
            request_builder = request_builder.header("Cookie", cookie.unwrap());
        }

        request_builder
            .send()
            .await
            .expect("Unable to send request.")
    }

    pub async fn get_runners(
        &self,
        page_number: i32,
        search_category: &str,
        search_keyword: &str,
        show_only_bsv: bool,
        cookie: Option<String>,
    ) -> Response {
        let mut request_builder = self
            .client
            .get(format!("{}/api/admin/runners", self.address))
            .query(&[
                ("page_number", page_number.to_string().as_ref()),
                ("search_category", search_category),
                ("search_keyword", search_keyword), // talisman-ignore-line
                ("show_only_bsv", show_only_bsv.to_string().as_ref()),
            ])
            .header("Content-Type", "application/json");

        if cookie.is_some() {
            request_builder = request_builder.header("Cookie", cookie.unwrap());
        }

        request_builder
            .send()
            .await
            .expect("Unable to send request")
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.database.get_connection()
    }

    pub fn get_email_server(&self) -> Option<&TestEmailServer> {
        self.email_server.as_ref()
    }
}

pub async fn extract_json_values(actual_response: Response) -> Map<String, serde_json::Value> {
    actual_response
        .json::<serde_json::Value>()
        .await
        .unwrap()
        .as_object()
        .unwrap()
        .clone()
}
