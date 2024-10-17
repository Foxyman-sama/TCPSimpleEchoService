// Написать сервис, прослушивающий сокеты (номер порта задается произвольно).
// Обработку обмена информацией с каждым клиентом реализовать в отдельном потоке.
// В журнал событий записывать данные о подключении и отключении клиентов.

use core::str;
use std::{
    env,
    fmt::format,
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
        let mut buf_reader = BufReader::new(&mut stream);
        let mut buffer = vec![];
        let result_of_reading = buf_reader.read_until(b'@', &mut buffer);

        match result_of_reading {
            Ok(size) => {
                if size == 0 {
                    return;
                }
            }
            _ => return,
        }

        buffer.pop();

        let parsed = str::from_utf8(&buffer).unwrap();
        println!("{}", parsed);

        let mut buf_writer = BufWriter::new(&mut stream);
        let answer = format!("Your answer: {}\n", parsed);
        let result_of_writing = buf_writer.write_all(answer.as_bytes());

        match result_of_writing {
            Ok(size) => (),
            _ => return,
        }
    }
}
