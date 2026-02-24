mod constants;
mod server;
mod error_handle;

fn main() {
  server::try_server_connect(constants::SERVER_PORT);
}
