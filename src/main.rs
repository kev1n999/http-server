mod constants;
mod server;

fn main() {
  server::try_server_connect(constants::SERVER_PORT);
}
