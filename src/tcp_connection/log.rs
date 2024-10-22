use core::str;
use std::{fs, io::Write, net::TcpStream};

const LOG_FILENAME: &'static str = "log.txt";

pub fn create_file() {
  let _ = fs::File::create_new(LOG_FILENAME);
}

pub fn log_event(stream: &TcpStream, status: &str) {
  let mut file = open_file();
  let _ = file.write_all(format_string(stream, status).as_bytes());
}

fn open_file() -> fs::File {
  fs::File::options().write(true).append(true).open(LOG_FILENAME).unwrap()
}

fn format_string(stream: &TcpStream, status: &str) -> String {
  let connection_ip = stream.peer_addr().unwrap().to_string();
  let current_time = chrono::Local::now();
  format!("User [{}] {} at {:?}\n", connection_ip, status, current_time)
}
