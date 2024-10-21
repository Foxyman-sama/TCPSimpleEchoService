use core::str;
use std::{
  env, fs,
  io::{BufRead, BufReader, BufWriter, Write},
  net::{TcpListener, TcpStream},
};

pub fn read_bytes(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
  let mut buf_reader = BufReader::new(stream);
  let mut result = vec![];
  buf_reader.read_until(b'@', &mut result)?;
  Ok(result)
}

pub fn write_string(stream: &mut TcpStream, parsed: &str) -> Result<(), std::io::Error> {
  let mut buf_writer = BufWriter::new(stream);
  let responce = format!("Your request: {}@", parsed);
  buf_writer.write_all(responce.as_bytes())
}
