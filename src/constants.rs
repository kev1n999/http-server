use std::env;

pub const SERVER_PORT: &'static str = env!("SERVER_PORT", "an error ocurred to read the server_port value in env");
