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
  // buffer to read data sent by client
  let mut buffer = [0u8; 4064];
  let n = stream.read(&mut buffer).unwrap();
  let request_str = String::from_utf8_lossy(&buffer[..n]);
  // get parts of request to separate headers of body
  let req_parts: Vec<&str> = request_str.split("\r\n\r\n").collect();

  // request headers
  let req_headers = req_parts[0];
  // request body
  let req_body = req_parts.get(1).unwrap_or(&"").trim();
  // request line
  // METHOD /PATH HTTP/1.1
  let request_line: Option<&str> = req_headers.lines().next();
  // display headers and body of request
  println!("new request:\nheaders:{:#?}\nbody: {:#?}\n", req_headers, req_body);

  let success_response_status_line = "HTTP/1.1 200 OK";
  let mut request_method = String::new();
  let mut request_path = String::new();

  // get and store method and path of request line to use to process requests
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

  let html_content_type = "text/html; charset=utf-8";
  // to store filenames to send to client/browser
  let mut filename = String::new();
  let server_response = Response::new_response(success_response_status_line.to_string(), html_content_type.to_string());
  let method = server_response.parse_method(&request_method);
  let route = Route::new_route(&method, &request_path);

  // process and respond requests
  match route.method {
    RequestMethod::GET => {
      match route.path.as_str() {
        "/" => filename.push_str("home.html"),
        "/calculator" => filename.push_str("calc.html"),
        "/scripts.js" => {
          // get the javascript content of file
          let js = fs::read_to_string("src/public/scripts.js").unwrap();
          format_response(&stream, success_response_status_line, "application/javascript", &format!("console.log('hello world!');\n{}", &js));
        }
        _ => {
          send_response_error(&stream, ResponseError::not_found_error("page not found!").content);
        },
      }
    },
    RequestMethod::POST => {
      // get the json body sent in the post request
      let json_body = req_body.trim_matches(|c: char| c.is_whitespace());
      // parse original json body to string
      let json_deserialize = serde_json::to_string_pretty(json_body);
      match route.path.as_str() {
        "/calculator" => {
          if let Ok(result) = json_deserialize {
            let inner_json: Option<String> = serde_json::from_str(&result.to_string()).unwrap();
            match inner_json {
              Some(j) => {
                // parse json to struct
                let parsed: CalcRequest = serde_json::from_str(&j).unwrap();
                let operation = parsed.operation;
                let number1 = parsed.number1;
                let number2 = parsed.number2;

                match operation.as_str() {
                  "sum" => {
                    let sum = number1 + number2;
                    format_response(&stream, success_response_status_line, "text/plain", &format!("the sum is: {}", sum));
                  },
                  "sub" => {
                    let sub = number1 - number2;
                    format_response(&stream, success_response_status_line, "text/plain", &format!("the sub is: {}", sub));
                  },
                  "mult" => {
                    let mult = number1 * number2;
                    format_response(&stream, success_response_status_line, "text/plain", &format!("the mult is: {}", mult));
                  },
                  "div" => {
                    let div = number1 / number2;
                    format_response(&stream, success_response_status_line, "text/plain", &format!("the div is: {}", div));
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

  // that html content will be sent in the response body
  // |------------------------------------------------------|
  // | Content-Type: text/html                              |
  // | Content-Length: size of html content                 |
  // | Status-Line/Response-Line: HTTP/1.1 200 OK           |
  // | -----------------------------------------------------|
  // | Body: html content                                   |
  // |------------------------------------------------------|

  let html_file_content = read_html_file(&filename);
  match html_file_content {
    Ok(response_content) => {
      let response = format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", success_response_status_line, html_content_type, response_content.len(), response_content);
      stream.write_all(response.as_bytes()).unwrap();
    },
    Err(err) => eprintln!("\nan error ocurred to get response content\n{}", err),
  }
}

fn read_html_file(file_name: &str) -> std::io::Result<String> {
  let mut html_file = fs::File::open("src/public/".to_string() + file_name)?;
  let mut html_content = String::new();
  html_file.read_to_string(&mut html_content)?;
  Ok(html_content)
}

fn format_response(mut stream: &TcpStream, status_line: &str, content_type: &str, content: &str) {
  stream.write_all(
    &format!(
      "{}\r\n\
       Content-Length: {}\r\n\
       Content-Type: {}\r\n\
       \r\n\
       {}", status_line, &format!("{}", content).len(), content_type.trim(), content,
    ).as_bytes()
  ).unwrap()
}
