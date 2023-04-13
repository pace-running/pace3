use crate::core::repository::UserRepository;
use crate::models::users::User;

pub trait UserService {
    fn find_user_by_username(&self, username: String) -> Option<User>;
    fn set_password(&self, username: String, new_password: String) -> anyhow::Result<()>; // talisman-ignore-line
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

    fn set_password(
        // talisman-ignore-line
        &self,
        username: String,
        new_password: String,
    ) -> anyhow::Result<()> {
        self.user_repository.set_password(username, new_password) // talisman-ignore-line
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::MockUserRepository;
    use mockall::predicate::*;

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

        assert_eq!(result, Some(User::default()))
    }

    #[test]
    fn set_password_must_call_repository() {
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_set_password()
            .with(eq(User::default().username), eq("new_password".to_string())) // talisman-ignore-line
            .times(1)
            .returning(|_r, _s| Ok(()));

        let user_service = DefaultUserService::new(user_repository);
        let result = user_service
            .set_password(User::default().username, "new_password".to_string()) // talisman-ignore-line
            .unwrap();

        assert_eq!(result, ())
    }
}
