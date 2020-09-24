use std::fmt;
use std::string::ToString;
use actix_web::http::StatusCode;
use actix_web::error::Error as ActixError;
use actix_web::{ResponseError, HttpResponse};

#[derive(Debug)]
pub struct Error {
  pub message: String,
  pub status_code: StatusCode,
}

impl Error {
  pub fn new(status_code: u16, message: &str) -> Self {
    Self {
      status_code: StatusCode::from_u16(status_code).unwrap(),
      message: message.to_string(),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    use std::io::ErrorKind as IOErrorKind;

    let status_code = match err.kind() {
      IOErrorKind::NotFound => StatusCode::NOT_FOUND,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    Self {
      status_code,
      message: err.to_string(),
    }
  }
}

impl From<ActixError> for Error {
  fn from(error: ActixError) -> Self {
    Self {
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      message: error.to_string(),
    }
  }
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code)
      .body(&self.message)
  }
}
