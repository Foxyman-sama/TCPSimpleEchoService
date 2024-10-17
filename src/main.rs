// Написать сервис, прослушивающий сокеты (номер порта задается произвольно).
// Обработку обмена информацией с каждым клиентом реализовать в отдельном потоке.
// В журнал событий записывать данные о подключении и отключении клиентов.

use core::{fmt, str};
use std::{
    env,
    fmt::{format, Error},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).unwrap();
    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        std::thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = match read_bytes(&mut stream) {
            Some(value) => value,
            None => return,
        };

        buffer.pop();

        let parsed = str::from_utf8(&buffer).unwrap();
        println!("{}", parsed);

        match write_string(&mut stream, parsed) {
            Ok(_) => (),
            _ => return,
        }
    }
}

fn read_bytes(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buf_reader = BufReader::new(stream);
    let mut buffer = vec![];
    let result_of_reading = buf_reader.read_until(b'@', &mut buffer);

    match result_of_reading {
        Ok(size) => {
            if size == 0 {
                return None;
            }
        }
        _ => return None,
    }

    Some(buffer)
}

fn write_string(stream: &mut TcpStream, parsed: &str) -> Result<(), std::io::Error> {
    let mut buf_writer = BufWriter::new(stream);
    let responce = format!("Your request: {}\n", parsed);
    buf_writer.write_all(responce.as_bytes())
}
