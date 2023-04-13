use diesel::RunQueryDsl;
use pace::core::repository::UserRepository;
use pace::get_connection_pool;
use pace::models::users::User;
use pace::repository::PostgresUserRepository;

#[test]
fn find_user_by_username_should_return_user_with_given_username_if_present_in_db() {
    let pool = get_connection_pool().expect("Unable to get connection pool.");
    let _ = diesel::sql_query(
        r#"
INSERT INTO users(username,password_hash,role)
VALUES(
  'admin',
  '$argon2i$v=19$m=4096,t=3,p=1$eUdkUHNOSTBLOXkyTmRpaw$G657jZT/Gk+Ipb4fFL/Ly9fh0Sv10YM7zwYvD2UiFTc',
  'admin'
)"#,
    )
    .execute(&mut pool.get().expect("Unable to get connection."))
    .expect("No value in result");

    let user_repository = PostgresUserRepository::new(pool.clone());

    let result = user_repository
        .find_user_by_username("admin".to_string())
        .expect("Unable to find user.");
    assert_eq!(result.id, 1)
}

#[test]
fn set_password_should_return_ok_on_success() {
    let pool = get_connection_pool().expect("Unable to get connection pool.");
    let _ = diesel::sql_query(
        r#"
INSERT INTO users(username,password_hash,role)
VALUES(
  'testi_mctest',
  '$argon2i$v=19$m=4096,t=3,p=1$eUdkUHNOSTBLOXkyTmRpaw$G657jZT/Gk+Ipb4fFL/Ly9fh0Sv10YM7zwYvD2UiFTc',
  'admin'
)"#,
    )
    .execute(&mut pool.get().expect("Unable to get connection."))
    .expect("No value in result");

    let user_repository = PostgresUserRepository::new(pool.clone());

    let result = user_repository
        .set_password("testi_mctest".to_string(), "new_password".to_string())
        .expect("Unable to change password.");

    assert_eq!(result, ())
}

#[test]
fn set_password_should_return_error_if_username_was_not_found() {
    let pool = get_connection_pool().expect("Unable to get connection pool.");

    let user_repository = PostgresUserRepository::new(pool.clone());

    let result =
        user_repository.set_password("does_not_exist".to_string(), "new_password".to_string());

    assert!(result.is_err())
}
