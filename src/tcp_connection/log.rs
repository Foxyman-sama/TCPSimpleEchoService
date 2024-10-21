use core::str;
use std::{fs, io::Write, net::TcpStream};

const LOG_FILENAME: &'static str = "log.txt";

pub fn create_file() {
  let _ = fs::File::create_new(LOG_FILENAME);
}

pub fn log_event(stream: &TcpStream, status: &str) {
  let mut file = fs::File::options().write(true).append(true).open(LOG_FILENAME).unwrap();
  let connection_ip = stream.peer_addr().unwrap().to_string();
  let current_time = chrono::Utc::now();
  let record = format!("User [{}] {} at {:?}\n", connection_ip, status, current_time);
  let _ = file.write_all(record.as_bytes());
}
