use std::fs;
use std::thread;
use std::io::{prelude::*, BufReader};
use std::net::{TcpStream, TcpListener};

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

  let status_line = "HTTP/1.1 200 OK";
  match read_html_file() {
    Ok(content) => {
      let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{content}", content.len());
      stream.write_all(response.as_bytes()).unwrap();
    },
    Err(err) => eprintln!("an error ocurred to send html response: {}", err),
  }
}

fn read_html_file() -> std::io::Result<String> {
  let mut html_file = fs::File::open("src/index.html")?;
  let mut html_content = String::new();
  html_file.read_to_string(&mut html_content)?;
  Ok(html_content)
}
