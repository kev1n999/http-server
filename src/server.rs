use std::fs;
use std::thread;
use std::io::{prelude::*};
use std::net::{TcpStream, TcpListener};
use crate::error_handle::ResponseError;
use crate::error_handle::send_response_error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CalcRequest {
  operation: String,
  number1: f64,
  number2: f64,
}
enum RequestMethod {
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
}
struct Route<'a> {
  method: &'a RequestMethod,
  path: &'a String,
}
struct Response {
  status_line: String,
  content_type: String,
}

impl Response {
  fn new_response(
    status_line: String, content_type: String,
  ) -> Self {
    Response { status_line, content_type }
  }
  fn parse_method(&self, method: &str) -> RequestMethod {
    match method {
      "GET" => RequestMethod::GET,
      "POST" => RequestMethod::POST,
      "PUT" => RequestMethod::PUT,
      "PATCH" => RequestMethod::PATCH,
      "DELETE" => RequestMethod::DELETE,
      _ => panic!("invalid request method!"),
    }
  }
}

impl<'a> Route<'a> {
  fn new_route(method: &'a RequestMethod, path: &'a String) -> Self {
    Route { method, path }
  }
}

// main function to connect server
pub fn try_server_connect(port: &'static str) {
  // create a tcp socket in localhost
  let stream = TcpListener::bind(&format!("127.0.0.1:{}", port)).unwrap();
  for stream in stream.incoming() {
    match stream {
      Ok(s) => {
        println!("connected!");
        thread::spawn(|| { server_handle(s); });
      },
      Err(err) => panic!("an error ocurred to try the server connect!\n{}", err),
    }
  }
}

pub fn server_handle(mut stream: TcpStream) {
  let mut buffer = [0u8; 4064];
  let n = stream.read(&mut buffer).unwrap();
  let request_str = String::from_utf8_lossy(&buffer[..n]);
  let req_parts: Vec<&str> = request_str.split("\r\n\r\n").collect();

  let req_headers = req_parts[0];
  let req_body = req_parts.get(1).unwrap_or(&"").trim();
  let request_line: Option<&str> = req_headers.lines().next();
  println!("new request:\nheaders:{:#?}\nbody: {:#?}\n", req_headers, req_body);

  let status_line = "HTTP/1.1 200 OK";
  let mut request_method = String::new();
  let mut request_path = String::new();

  match request_line {
    Some(line) => {
      let mut parts = line.split_whitespace();
      if let Some(method) = parts.next() {
        request_method = method.to_string();
      }
      if let Some(path) = parts.next() {
        request_path = path.to_string();
      }
    },
    _ => eprintln!("an error ocurred to read the request line!"),
  }

  let content_type = "text/html; charset=utf-8";
  let mut filename = String::new();
  let server_response = Response::new_response(status_line.to_string(), content_type.to_string());
  let method = server_response.parse_method(&request_method);
  let route = Route::new_route(&method, &request_path);

  match route.method {
    RequestMethod::GET => {
      match route.path.as_str() {
        "/" => filename.push_str("home.html"),
        "/calculator" => filename.push_str("calc.html"),
        _ => {
          send_response_error(&stream, ResponseError::not_found_error("page not found!").content);
        },
      }
    },
    RequestMethod::POST => {
      let json_body = req_body.trim_matches(|c: char| c.is_whitespace());
      let json_deserialize = serde_json::to_string_pretty(json_body);
      match route.path.as_str() {
        "/calculator" => {
          if let Ok(result) = json_deserialize {
            let inner_json: Option<String> = serde_json::from_str(&result.to_string()).unwrap();
            match inner_json {
              Some(j) => {
                let parsed: CalcRequest = serde_json::from_str(&j).unwrap();
                let operation = parsed.operation;
                let number1 = parsed.number1;
                let number2 = parsed.number2;

                match operation.as_str() {
                  "sum" => {
                    let sum = number1 + number2;
                    calculator_response(&stream, status_line, &format!("{}", sum));
                  },
                  "sub" => {
                    let sub = number1 - number2;
                    calculator_response(&stream, status_line, &format!("{}", sub));
                  },
                  "mult" => {
                    let mult = number1 * number2;
                    calculator_response(&stream, status_line, &format!("{}", mult));
                  },
                  "div" => {
                    let div = number1 / number2;
                    calculator_response(&stream, status_line, &format!("{}", div));
                  }
                  _ => {}
                }
              },
              None => eprintln!("any json received"),
            }
          }
        },
        _ => {
          send_response_error(&stream, ResponseError::not_found_error("route not found!").content);
        },
      }
    }
    _ => eprintln!("invalid method!"),
  }

  let html_file_content = read_html_file(&filename);
  match html_file_content {
    Ok(response_content) => {
      let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, response_content.len(), response_content);
      stream.write_all(response.as_bytes()).unwrap();
    },
    Err(err) => eprintln!("\nan error ocurred to get response content\n{}", err),
  }
}

fn read_html_file(file_name: &str) -> std::io::Result<String> {
  let mut html_file = fs::File::open("src/html/".to_string() + file_name)?;
  let mut html_content = String::new();
  html_file.read_to_string(&mut html_content)?;
  Ok(html_content)
}

fn calculator_response(mut stream: &TcpStream, status_line: &str, result: &str) {
  stream.write_all(
    &format!(
      "{}\r\n\
       Content-Length: {}\r\n\
       Content-Type: text/plain\r\n\
       \r\n\
       {}", status_line, &format!("{}", result).len(), result,
    ).as_bytes()
  ).unwrap()
}
