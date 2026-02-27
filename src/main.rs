mod constants;
mod server;
mod response_message;

fn main() {
  server::try_server_connect(constants::SERVER_PORT);
}
