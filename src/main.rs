// Написать сервис, прослушивающий сокеты (номер порта задается произвольно).
// Обработку обмена информацией с каждым клиентом реализовать в отдельном потоке.
// В журнал событий записывать данные о подключении и отключении клиентов.

use core::{fmt, str};
use std::{
  env,
  fmt::{format, Error},
  fs,
  io::{BufRead, BufReader, BufWriter, Read, Write},
  net::{TcpListener, TcpStream},
  time::SystemTime,
};

const LOG_FILENAME: &'static str = "log.txt";

fn main() {
  let args: Vec<String> = env::args().collect();
  let port = args.get(1).unwrap();
  let addr = format!("localhost:{}", port);
  let listener = TcpListener::bind(addr).unwrap();
  let mut file = fs::File::create(LOG_FILENAME).unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    let connection_ip = stream.peer_addr().unwrap().to_string();
    let current_time = chrono::Utc::now();
    let record = format!("User [{}] connected at {:?}\n", connection_ip, current_time);
    file.write_all(record.as_bytes());

    std::thread::spawn(move || {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  loop {
    let buffer = match read_bytes(&mut stream) {
      Ok(mut buffer) => {
        if buffer.len() == 0 {
          return;
        }

        buffer.pop();
        buffer
      }
      _ => return,
    };

    let parsed = str::from_utf8(&buffer).unwrap();
    println!("{}", parsed);

    match write_string(&mut stream, parsed) {
      Ok(_) => (),
      _ => return,
    }
  }
}

fn read_bytes(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
  let mut buf_reader = BufReader::new(stream);
  let mut result = vec![];
  buf_reader.read_until(b'@', &mut result)?;
  Ok(result)
}

fn write_string(stream: &mut TcpStream, parsed: &str) -> Result<(), std::io::Error> {
  let mut buf_writer = BufWriter::new(stream);
  let responce = format!("Your request: {}\n", parsed);
  buf_writer.write_all(responce.as_bytes())
}
