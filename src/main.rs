// Написать сервис, прослушивающий сокеты (номер порта задается произвольно).
// Обработку обмена информацией с каждым клиентом реализовать в отдельном потоке.
// В журнал событий записывать данные о подключении и отключении клиентов.

use core::str;
use std::{
  env, fs,
  io::{BufRead, BufReader, BufWriter, Write},
  net::{TcpListener, TcpStream},
};

const LOG_FILENAME: &'static str = "log.txt";

fn main() {
  let args: Vec<String> = env::args().collect();
  let port = args.get(1).unwrap();
  let addr = format!("localhost:{}", port);
  let listener = TcpListener::bind(addr).unwrap();

  let _ = fs::File::create_new(LOG_FILENAME);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    log_event(&stream, "connected");

    std::thread::spawn(move || {
      let stream = handle_connection(stream);
      log_event(&stream, "disconnected");
    });
  }
}

fn log_event(stream: &TcpStream, status: &str) {
  let mut file = fs::File::options().write(true).append(true).open(LOG_FILENAME).unwrap();
  let connection_ip = stream.peer_addr().unwrap().to_string();
  let current_time = chrono::Utc::now();
  let record = format!("User [{}] {} at {:?}\n", connection_ip, status, current_time);
  let _ = file.write_all(record.as_bytes());
}

fn handle_connection(mut stream: TcpStream) -> TcpStream {
  loop {
    let buffer = match read_bytes(&mut stream) {
      Ok(mut buffer) => {
        if buffer.len() == 0 {
          break;
        }

        buffer.pop();
        buffer
      }
      _ => break,
    };

    let parsed = str::from_utf8(&buffer).unwrap();
    println!("{}", parsed);

    match write_string(&mut stream, parsed) {
      Ok(_) => (),
      _ => break,
    }
  }

  stream
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
