use std::fs;
use std::thread;
use std::io::{prelude::*, BufReader};
use std::net::{TcpStream, TcpListener};
use crate::error_handle::ResponseError;

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
  // body: HashMap<String, String>,
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

pub fn try_server_connect(port: &'static str) {
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
  let buff_reader = BufReader::new(&stream);
  let request: Vec<_> = buff_reader.lines()
    .map(|result| result.expect("an error ocurred to get result of request!"))
    .take_while(|line| !line.is_empty()) // empty line to separate header of body
    .collect();
  println!("new request: {:#?}", request);

  let request_line = request.get(0);
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
        _ => {
          let response_error = ResponseError::not_found_error("page not found!");
          let response_content = response_error.content;
          stream.write_all(response_content.as_bytes()).unwrap();
        },
      }
    },
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
