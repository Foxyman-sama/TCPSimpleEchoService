use core::str;
use std::{
  io::{BufRead, BufReader, BufWriter, Write},
  net::TcpStream,
};

pub fn read_bytes(stream: &mut TcpStream) -> Vec<u8> {
  let mut buf_reader = BufReader::new(stream);
  let mut result = vec![];
  let _ = buf_reader.read_until(b'@', &mut result);
  result
}

pub fn write_string(stream: &TcpStream, parsed: &str) {
  let mut buf_writer = BufWriter::new(stream);
  let responce = format!("Your request: {}@", parsed);
  let _ = buf_writer.write_all(responce.as_bytes());
}
