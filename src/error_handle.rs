use std::fmt;

pub enum StatusCode {
  Success(i32),
  NotFound(i32),
}

pub struct ResponseError {
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
    write!(f, "Message: {}", self.content)
  }
}

impl ResponseError {
  pub fn not_found_error(content_error: &str) -> Self {
    let def_message = format!(
        "HTTP/1.1 {} Not Found\r\n\
    Content-Type: text/plain; charset=utf-8\r\n\
    Content-Length: {}\r\n\
    Connection: close\r\n\
    \r\n\
    {}",
        StatusCode::NotFound(404),
        content_error.len(),
        content_error
    );
    ResponseError { content: def_message }
  }
}
