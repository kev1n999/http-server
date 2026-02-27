mod constants;
mod server;
mod response_message;
mod database;

fn main() {
  server::try_server_connect(constants::SERVER_PORT);
}
