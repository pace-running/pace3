use argon2::{self};
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

impl PartialEq<LoginData> for User {
    fn eq(&self, other: &LoginData) -> bool {
        let matches = argon2::verify_encoded(&self.password_hash, other.password.as_bytes());
        matches.unwrap_or(false) && self.username == other.username
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: "".to_string(),
            password_hash: "".to_string(),
            role: "".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct PasswordChangeData {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub username: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub result: String,
    pub error_message: Option<String>,
}

impl From<&User> for LoginResponse {
    fn from(user: &User) -> Self {
        LoginResponse {
            username: user.username.to_string(),
            role: user.role.to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::models::users::{LoginData, LoginResponse, User};

    #[test]
    fn unit_matching_logindata_and_user() {
        let user = User {
            id: 0,
            username: "admin".to_string(),
            password_hash: "$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$OOA07UjKrh3ijWboNB5/Ur274nxXirUuifmSuGCXwY0".to_string(),
            role: "unused_role".to_string(),
        };
        let data = LoginData {
            username: "admin".to_string(),
            password: "xoh7Ongui4oo".to_string(),
        };
        assert!(user == data)
    }
    #[test]
    fn unit_not_matching_usernames() {
        let user = User {
            id: 0,
            username: "admin".to_string(),
            password_hash: "$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$OOA07UjKrh3ijWboNB5/Ur274nxXirUuifmSuGCXwY0".to_string(),
            role: "asdf".to_string(),
        };
        let data = LoginData {
            username: "non-admin".to_string(),
            password: "xoh7Ongui4oo".to_string(),
        };
        assert!(user != data)
    }
    #[test]
    fn unit_not_matching_password() {
        let user = User {
            id: 0,
            username: "admin".to_string(),
            password_hash: "$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$OOA07UjKrh3ijWboNB5/Ur274nxXirUuifmSuGCXwY0".to_string(),
            role: "asdf".to_string(),
        };
        let data = LoginData {
            username: "admin".to_string(),
            password: "invalid password".to_string(),
        };
        assert!(user != data)
    }

    #[test]
    fn unit_converts_user_to_login_response() {
        let user = User {
            id: 0,
            username: "admin".to_string(),
            password_hash: "whatever".to_string(),
            role: "asdf".to_string(),
        };
        let login_response = LoginResponse::from(&user);
        assert_eq!(login_response.role, "asdf".to_string())
    }
}
