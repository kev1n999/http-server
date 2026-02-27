use std::fs;
use std::thread;
use std::io::{prelude::*};
use std::net::{TcpStream, TcpListener};
use crate::response_message::{ResponseMessage, StatusCode};
use crate::response_message::send_response_error;
use crate::database;
use serde::Deserialize;
use std::collections;

enum RequestMethod {
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
  Invalid,
}

#[derive(Debug, Deserialize)]
struct People {
  name: String,
  age: String,
}
#[derive(Debug, Deserialize)]
struct CalcRequest {
  operation: String,
  number1: f64,
  number2: f64,
}
struct Request {
  method: RequestMethod,
  path: String,
  headers: collections::HashMap<String, String>,
}
struct Route<'a> {
  method: &'a RequestMethod,
  path: &'a String,
}
struct Response {
  status_code: StatusCode,
  headers: collections::HashMap<String, String>,
}

impl RequestMethod {
  fn parse_request_method(str_method: &str) -> Self {
    match str_method {
      "GET" => Self::GET,
      "POST" => Self::POST,
      "PUT" => Self::PUT,
      "PATCH" => Self::PATCH,
      "DELETE" => Self::DELETE,
      _ => Self::Invalid,
    }
  }
}
impl Response {
  fn new_response(status_code: StatusCode, headers: collections::HashMap<String, String>) -> Self {
    Response { status_code: status_code, headers: headers, }
  }
  fn parse_response(&self, body_content: &str) -> Vec<u8> {
    let mut response = String::new();
    response.push_str(&format!("HTTP/1.1 {}\r\n", self.status_code));
    for (k, v) in &self.headers {
      response.push_str(&format!("{}: {}\r\n", k, v));
    }
    response.push_str("\r\n");
    response.push_str(body_content);
    response.into_bytes()
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

  let success_status_code = StatusCode::Success;
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
  let js_content_type = "application/javascript";
  let text_content_type = "text/plain";
  let json_content_type = "application/json";
  // to store filenames to send to client/browser
  let mut filename = String::new();

  // process and respond requests
  match RequestMethod::parse_request_method(&request_method) {
    RequestMethod::GET => {
      match request_path.as_str() {
        "/" => filename.push_str("home.html"),
        "/calculator" => filename.push_str("calc.html"),
        "/create-people" => filename.push_str("people.html"),
        "/people.js" => {
          // get the javascript content of file
          let js = fs::read_to_string("src/public/people.js").unwrap();
          let body_content = &format!("console.log('people.js was loaded!!');\n{}", &js);
          let response = Response::new_response(
            success_status_code, create_response_header(js_content_type, body_content)
          );
          stream.write_all(&response.parse_response(body_content));
        }
        "/calc.js" => {
          // get the javascript content of file
          let js = fs::read_to_string("src/public/calc.js").unwrap();
          let body_content = &format!("console.log('hello world!');\n{}", &js);
          let response = Response::new_response(
            success_status_code, create_response_header(js_content_type, body_content)
          );
          stream.write_all(&response.parse_response(body_content));
        }
        _ => {
          send_response_error(&stream, ResponseMessage::not_found_error("page not found!", text_content_type).content);
        },
      }
    },
    RequestMethod::POST => {
      // get the json body sent in the post request
      let json_body = req_body.trim_matches(|c: char| c.is_whitespace());
      // parse original json body to string
      let json_deserialize = serde_json::to_string_pretty(json_body);
      match request_path.as_str() {
        "/create-people" => {
          if let Ok(result) = json_deserialize {
            let inner_json: Option<String> = serde_json::from_str(&result.to_string()).unwrap();
            match inner_json {
              Some(j) => {
                let people_parsed: People = serde_json::from_str(&j).unwrap();
                if let Ok(conn) = database::connect_db() {
                  match database::insert_new_people(conn, &people_parsed.name, &people_parsed.age) {
                    Ok(_) => {
                      let body_content = &format!("People was created!");
                      let response = Response::new_response(
                        success_status_code, create_response_header(text_content_type, body_content)
                      );
                      stream.write_all(&response.parse_response(body_content));
                    },
                    Err(err) => {
                      eprintln!("an error ocurred to create the people!");
                      let body_content = format!( r#"{{ "status": "err", "message": "{}" }}"#, err);
                      send_response_error(&stream, ResponseMessage::badrequest_error(&body_content, json_content_type).content);
                    }
                  }
                }

              },
              _ => eprintln!("an error ocurred to parse json!")
            }
          }
        },
        "/calculator" => {
          if let Ok(result) = json_deserialize {
            let inner_json: Option<String> = serde_json::from_str(&result.to_string()).unwrap();
            match inner_json {
              Some(j) => {
                // parse json to struct
                let calc_parsed: CalcRequest = serde_json::from_str(&j).unwrap();
                let operation = calc_parsed.operation;
                let number1 = calc_parsed.number1;
                let number2 = calc_parsed.number2;

                match operation.as_str() {
                  "sum" => {
                    let sum = number1 + number2;
                    let body_content = &format!("the sum is: {}", sum);
                    let response = Response::new_response(
                      success_status_code, create_response_header(text_content_type, body_content)
                    );
                    stream.write_all(&response.parse_response(body_content));
                  },
                  "sub" => {
                    let sub = number1 - number2;
                    let body_content = &format!("the sub is: {}", sub);
                    let response = Response::new_response(
                      success_status_code, create_response_header(text_content_type, body_content)
                    );
                    stream.write_all(&response.parse_response(body_content));
                  },
                  "mult" => {
                    let mult = number1 * number2;
                    let body_content = &format!("the mult is: {}", mult);
                    let response = Response::new_response(
                      success_status_code, create_response_header(text_content_type, body_content)
                    );
                    stream.write_all(&response.parse_response(body_content));
                  },
                  "div" => {
                    let div = number1 / number2;
                    let body_content = &format!("the div is: {}", div);
                    let response = Response::new_response(
                      success_status_code, create_response_header(text_content_type, body_content)
                    );
                    stream.write_all(&response.parse_response(body_content));
                  }
                  _ => eprintln!("an error ocurred to parse json!")
                }
              },
              None => eprintln!("any json received"),
            }
          }
        },
        _ => {
          send_response_error(&stream, ResponseMessage::not_found_error("route not found!", text_content_type).content);
        },
      }
    },
    _ => eprintln!("..."),
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
      let response = Response::new_response(success_status_code, create_response_header(html_content_type, &response_content));
      stream.write_all(&response.parse_response(&response_content));
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

fn create_response_header(content_type: &str, body_content: &str) -> collections::HashMap<String, String> {
  let mut headers_hash: collections::HashMap<String, String> = collections::HashMap::new();
  headers_hash.insert("Content-Length".to_string(), body_content.len().to_string());
  headers_hash.insert("Content-Type".to_string(), content_type.to_string());
  headers_hash
}
