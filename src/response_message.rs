use std::fmt;
use std::io::{Write};
use std::net::TcpStream;

#[derive(Copy, Clone)]
pub enum StatusCode {
  Success,
  Created,
  BadRequest,
  NotFound,
  InternalServerError,
}

pub struct ResponseMessage {
  pub content: String,
}

impl fmt::Display for StatusCode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StatusCode::Success => write!(f,  "200 OK"),
      StatusCode::Created => write!(f,  "201 Created"),
      StatusCode::BadRequest => write!(f,  "400 Bad Request"),
      StatusCode::NotFound => write!(f,  "404 Not Found"),
      StatusCode::InternalServerError => write!(f,  "500 Internal Server Error"),
    }
  }
}
impl fmt::Display for ResponseMessage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Message: {}", self.content)
  }
}

impl ResponseMessage {
  pub fn not_found_error(content_error: &str) -> Self {
    let def_message = format!(
        "HTTP/1.1 {}\r\n\
    Content-Type: text/plain; charset=utf-8\r\n\
    Content-Length: {}\r\n\
    Connection: close\r\n\
    \r\n\
    {}",
        StatusCode::NotFound,
        content_error.len(),
        content_error
    );
    ResponseMessage { content: def_message }
  }
}

pub fn send_response_error(mut stream: &TcpStream, response_content: String) {
  stream.write_all(&response_content.as_bytes()).unwrap();
}
