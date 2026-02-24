use std::fmt;

pub enum StatusCode {
  Success(i32),
  NotFound(i32),
}

pub struct ResponseError {
  pub status_code: StatusCode,
  pub content: String,
}

impl fmt::Display for StatusCode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Success(c) => write!(f, "{}", c),
      Self::NotFound(c) => write!(f, "{}", c),
    }
  }
}

impl fmt::Display for ResponseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Status Code: {}\nMessage: {}", self.status_code, self.content)
  }
}

impl ResponseError {
  pub fn not_found_error(content_error: &str) -> Self {
    ResponseError { status_code: StatusCode::NotFound(404), content: content_error.to_string() }
  }
}
