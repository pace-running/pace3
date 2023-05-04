use crate::validation::ValidationError;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Abstraction for displaying 4xx errors to users.
#[derive(Clone, PartialEq, Eq)]
pub enum ClientError {
    BadRequestError,                  // 400
    AuthenticationError,              // 401
    AuthorizationError,               // 403
    ResourceNotFoundError,            // 404
    ValidationError(ValidationError), // 422
}

impl Debug for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::BadRequestError => write!(f, "BadRequestError"),
            ClientError::AuthenticationError => write!(f, "AuthenticationError"),
            ClientError::AuthorizationError => write!(f, "AuthorizationError"),
            ClientError::ResourceNotFoundError => write!(f, "ResourceNotFoundError"),
            ClientError::ValidationError(e) => (e as &dyn Debug).fmt(f),
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::BadRequestError => {
                write!(
                    f,
                    "{}",
                    serde_json::json!({
                        "error_type": "Bad Request",
                        "error_message": "Bad data",
                        "status_code": "400",
                    })
                )
            }
            ClientError::AuthenticationError => {
                write!(
                    f,
                    "{}",
                    serde_json::json!({
                        "error_type": "Unauthorized",
                        "error_message": "",
                        "status_code": "401",
                    })
                )
            }
            ClientError::AuthorizationError => {
                write!(
                    f,
                    "{}",
                    serde_json::json!({
                        "error_type": "Forbidden",
                        "error_message": "You are not authorized to make this request.",
                        "status_code": "403",
                    })
                )
            }
            ClientError::ResourceNotFoundError => {
                write!(
                    f,
                    "{}",
                    serde_json::json!({
                        "error_type": "Not Found",
                        "error_message": "The resource you requested doesn't seem to exist.",
                        "status_code": "404",
                    })
                )
            }
            ClientError::ValidationError(e) => {
                write!(
                    f,
                    "{}",
                    serde_json::json!({
                        "error_type": "Unprocessable Entity",
                        "error_message": e.to_string(),
                        "error_details": {
                            "form": e.form(),
                            "field_errors": e.field_errors(),
                        },
                        "status_code": "422",
                    })
                )
            }
        }
    }
}

impl actix_web::error::ResponseError for ClientError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ClientError::BadRequestError => StatusCode::BAD_REQUEST,
            ClientError::AuthenticationError => StatusCode::UNAUTHORIZED,
            ClientError::AuthorizationError => StatusCode::FORBIDDEN,
            ClientError::ResourceNotFoundError => StatusCode::NOT_FOUND,
            ClientError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct InternalError<E>
where
    E: Debug + Display + 'static,
{
    inner_error: E,
}

impl<E> InternalError<E>
where
    E: Debug + Display + 'static,
{
    fn new(inner_error: E) -> Self {
        Self { inner_error }
    }
}

impl<E> From<E> for InternalError<E>
where
    E: Debug + Display + 'static,
{
    fn from(value: E) -> Self {
        InternalError::new(value)
    }
}

impl<E> Debug for InternalError<E>
where
    E: Debug + Display + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InternalError")
            .field("inner_error", &self.inner_error)
            .finish()
    }
}

impl<E> Display for InternalError<E>
where
    E: Debug + Display + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An internal error occurred. Please try again later.")
    }
}

impl<E> Error for InternalError<E> where E: Debug + Display + 'static {}

impl<E> actix_web::error::ResponseError for InternalError<E>
where
    E: Debug + Display + 'static,
{
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::json!({
            "error_type": "Internal Server Error",
            "error_message": self.to_string(),
            "status_code": self.status_code().to_string(),
        })
        .to_string();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }
}
