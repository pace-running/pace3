use actix_web::web::Json;
use diesel::r2d2::ConnectionManager;
use diesel::{pg::Pg, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use pace::handlers::theme::ThemeData;
use r2d2::{Pool, PooledConnection};
use reqwest::cookie::Cookie;
use reqwest::{Client, Response};
use serde_json::Map;
use std::env;
use std::error::Error;
use std::net::TcpListener;
use testcontainers::clients::Cli;
use testcontainers::images::postgres::Postgres;
use testcontainers::Container;

use pace::models::info::Info;
use pace::models::users::LoginData;
use pace::{get_connection_pool, run};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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
        env::set_var("POSTGRES_DB", pg_db);
        env::set_var("POSTGRES_USER", pg_user);
        env::set_var("POSTGRES_PASSWORD", pg_password); // talisman-ignore-line
        let database: Container<'a, Postgres> = docker.run(image);
        let pg_port = database.get_host_port_ipv4(5432);
        env::set_var(
            "DATABASE_URL",
            format!("postgres://{pg_user}:{pg_password}@127.0.0.1:{pg_port}/{pg_db}"), // talisman-ignore-line
        );
        let connection_pool = get_connection_pool().expect("Could not initialize connection pool");
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
    _database: TestDatabase<'a>,
    client: Client,
    address: String,
}

impl<'a> TestApp<'a> {
    pub async fn new(docker: &'a Cli) -> TestApp<'a> {
        let database = TestDatabase::with_migrations(docker);
        let client = Client::new();
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("Unable to bind random port.");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        let server =
            run(listener, database.connection_pool.clone()).expect("Unable to bind address");

        let _ = tokio::spawn(server);

        Self {
            _database: database,
            client,
            address,
        }
    }

    pub async fn create_app(&self) -> String {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("Unable to bind random port.");
        let port = listener.local_addr().unwrap().port();

        let server =
            run(listener, self._database.connection_pool.clone()).expect("Unable to bind address");

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

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self._database.get_connection()
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
