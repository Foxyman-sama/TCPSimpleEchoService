// Написать сервис, прослушивающий сокеты (номер порта задается произвольно).
// Обработку обмена информацией с каждым клиентом реализовать в отдельном потоке.
// В журнал событий записывать данные о подключении и отключении клиентов.

use std::{
    env,
    fmt::format,
    io::{BufRead, BufReader},
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
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
