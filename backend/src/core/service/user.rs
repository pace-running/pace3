use crate::core::repository::UserRepository;
use crate::models::users::User;

pub trait UserService {
    fn find_user_by_username(&self, username: String) -> Option<User>;
    fn change_password(
        &self,
        username: String,
        current_password: String,
        new_password: String,
    ) -> anyhow::Result<()>;
}

fn hash_password(password: &str, // talisman-ignore-line
) -> String {
    use rand::Rng;

    let config = argon2::Config::default();
    let salt: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    argon2::hash_encoded(
        password.as_bytes(), // talisman-ignore-line
        salt.as_bytes(),
        &config,
    )
    .unwrap()
}

fn password_matches_hash(
    hash: &str,
    password: &str, // talisman-ignore-line
) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).is_ok_and(|is_verified| is_verified)
}

pub struct DefaultUserService<UR: UserRepository> {
    user_repository: UR,
}

impl<UR: UserRepository> DefaultUserService<UR> {
    pub fn new(user_repository: UR) -> Self {
        DefaultUserService { user_repository }
    }
}

impl<UR: UserRepository> UserService for DefaultUserService<UR> {
    fn find_user_by_username(&self, username: String) -> Option<User> {
        self.user_repository.find_user_by_username(username)
    }

    fn change_password(
        // talisman-ignore-line
        &self,
        username: String,
        current_password: String,
        new_password: String,
    ) -> anyhow::Result<()> {
        let current_password_is_valid = self
            .find_user_by_username(username.clone())
            .ok_or(anyhow::Error::msg("User not found."))
            .map(|user| password_matches_hash(&user.password_hash, &current_password))?; // talisman-ignore-line

        if current_password_is_valid {
            let new_password_hash = hash_password(&new_password); // talisman-ignore-line
            self.user_repository
                .set_password_hash(username, new_password_hash) // talisman-ignore-line
        } else {
            Err(anyhow::Error::msg("Password invalid."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::MockUserRepository;
    use mockall::predicate::*;

    const EXAMPLE_PASSWORD: &'static str = "xoh7Ongui4oo"; // talisman-ignore-line

    /// Example hash for `xoh7Ongui4oo`.
    const EXAMPLE_PASSWORD_HASH: &'static str = // talisman-ignore-line
        "$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$OOA07UjKrh3ijWboNB5/Ur274nxXirUuifmSuGCXwY0"; // talisman-ignore-line

    #[test]
    fn find_user_by_username_must_call_repository() {
        let mut user_repository = MockUserRepository::new();

        user_repository
            .expect_find_user_by_username()
            .with(eq(User::default().username))
            .times(1)
            .returning(|_r| Some(User::default()));

        let user_service = DefaultUserService::new(user_repository);

        let result = user_service.find_user_by_username(User::default().username);

        assert_eq!(result, Some(User::default()));
    }

    #[test]
    fn change_password_must_fail_if_current_password_is_wrong() {
        let example_user: User = User {
            id: 73,
            username: "example_user".to_string(),
            password_hash: EXAMPLE_PASSWORD_HASH.to_string(), // talisman-ignore-line
            role: "admin".to_string(),
        };

        let mut user_repository = MockUserRepository::new();

        user_repository
            .expect_find_user_by_username()
            .with(eq("example_user".to_string()))
            .times(1)
            .returning(move |_r| Some(example_user.clone()));

        user_repository.expect_set_password_hash().never();

        let user_service = DefaultUserService::new(user_repository);
        let result = user_service.change_password(
            "example_user".to_string(),
            "invalid_value".to_string(),
            "new_password".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn change_password_must_not_fail_if_current_password_is_correct() {
        let example_user: User = User {
            id: 73,
            username: "example_user".to_string(),
            password_hash: EXAMPLE_PASSWORD_HASH.to_string(), // talisman-ignore-line
            role: "admin".to_string(),
        };

        let mut user_repository = MockUserRepository::new();

        user_repository
            .expect_find_user_by_username()
            .with(eq("example_user".to_string()))
            .times(1)
            .returning(move |_r| Some(example_user.clone()));

        user_repository
            .expect_set_password_hash()
            .with(
                eq("example_user".to_string()),
                function(|new_hash: &String| password_matches_hash(&new_hash, "new_password")), // talisman-ignore-line
            )
            .times(1)
            .returning(|_r, _s| Ok(()));

        let user_service = DefaultUserService::new(user_repository);
        let result = user_service.change_password(
            "example_user".to_string(),
            EXAMPLE_PASSWORD.to_string(), // talisman-ignore-line
            "new_password".to_string(),
        );

        assert!(result.is_ok());
    }
}
