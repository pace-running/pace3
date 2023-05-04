use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait Validate: Sized {
    /// Performs the validation.
    fn validate(self) -> Result<Self, ValidationError>;
}

pub trait ValidateInto<T>: Sized
where
    T: TryInto<T>,
{
    /// Performs the validation and conversion.
    fn validate_into(self) -> Result<T, ValidationError>;
}

pub trait ValidateFrom<T>: Sized
where
    T: TryFrom<T>,
{
    /// Performs the validation and conversion.
    fn validate_from(value: T) -> Result<Self, ValidationError>;
}

// ValidateFrom implies Validate
impl<T, U> ValidateInto<U> for T
where
    U: ValidateFrom<T>,
{
    #[inline]
    fn validate_into(self) -> Result<U, ValidationError> {
        U::validate_from(self)
    }
}

// ValidateFrom for Validate implies that there won't be any conversion errors
// but only validation errors, if any.
impl<T> ValidateFrom<T> for T
where
    T: Validate,
{
    #[inline]
    fn validate_from(value: T) -> Result<Self, ValidationError> {
        value.validate()
    }
}

impl<T, U> ValidateFrom<actix_web::web::Json<U>> for T
where
    T: ValidateFrom<U>,
{
    #[inline]
    fn validate_from(value: actix_web::web::Json<U>) -> Result<T, ValidationError> {
        T::validate_from(value.into_inner())
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct ValidationError {
    form: String,
    field_errors: HashMap<String, Vec<String>>,
}

impl ValidationError {
    /// Creates a new ValidationError with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `form`: a string representing the form/struct
    /// * `field_errors`: a map of field names and messages explaining errors
    ///
    /// returns: ValidationError
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use pace::validation::ValidationError;
    /// let validation_error = ValidationError::new(
    ///     "registration",
    ///     HashMap::from([("username", vec!("INVALID_CHARS", "TOO_SHORT"))]),
    /// );
    /// ```
    pub fn new(
        form: impl Into<String>,
        field_errors: HashMap<impl Into<String>, Vec<impl Into<String>>>,
    ) -> ValidationError {
        Self {
            form: form.into(),
            field_errors: field_errors
                .into_iter()
                .map(|(k, v)| (k.into(), v.into_iter().map(|iv| iv.into()).collect()))
                .collect(),
        }
    }

    pub fn form(&self) -> &str {
        &self.form
    }

    pub fn field_errors(&self) -> &HashMap<String, Vec<String>> {
        &self.field_errors
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation errors in {}: {}",
            self.form,
            self.field_errors
                .iter()
                .map(|(k, v)| format!("field `{}` {}", k, v.join(", ")))
                .collect::<Vec<_>>()
                .join("; "),
        )
    }
}

impl Error for ValidationError {}
