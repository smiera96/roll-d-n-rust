use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum ValidationException {
    NotEmpty(String),
    Numeric(String),
}

impl fmt::Display for ValidationException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationException::NotEmpty(field) => write!(f, "The value of {} cannot be empty", field),
            ValidationException::Numeric(field)  => write!(f, "The value of {} must be numeric", field),
        }
    }
}

impl ResponseError for ValidationException {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self.to_string())
    }
}

pub(crate) fn validate_not_empty(value: &String, field_name: &str) -> Result<(), ValidationException> {
    if value.is_empty() {
        Err(ValidationException::NotEmpty(field_name.to_string()))
    } else {
        Ok(())
    }
}

pub(crate) fn validate_is_numeric(value: &String, field_name: &str) -> Result<(), ValidationException> {
    if value.chars().all(char::is_numeric) {
        Ok(())
    } else {
        Err(ValidationException::Numeric(field_name.to_string()))
    }
}
