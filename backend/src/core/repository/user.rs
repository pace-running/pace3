use crate::models::users::User;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait UserRepository {
    fn find_user_by_username(&self, username: String) -> Option<User>;
    fn set_password_hash(&self, username: String, new_password_hash: String) -> anyhow::Result<()>; // talisman-ignore-line
}
