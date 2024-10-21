use std::{
  env,
  net::{TcpListener, TcpStream},
};

pub mod log;
mod tcp_io;

pub fn start_handling_connection() {
  let listener = create_listener();
  log::create_file();
  listen(listener);
}

fn create_listener() -> TcpListener {
  TcpListener::bind(make_address()).unwrap()
}

fn make_address() -> String {
  let args: Vec<String> = env::args().collect();
  let port = args.get(1).unwrap();
  format!("localhost:{}", port)
}

fn listen(listener: TcpListener) {
  for stream in listener.incoming() {
    handle_connection(stream.unwrap());
  }
}

fn handle_connection(stream: TcpStream) {
  log::log_event(&stream, "connected");
  create_io_thread(stream);
}

fn create_io_thread(stream: TcpStream) {
  std::thread::spawn(move || {
    let stream = handle_io(stream);
    log::log_event(&stream, "disconnected");
  });
}

fn handle_io(mut stream: TcpStream) -> TcpStream {
  loop {
    let buffer = tcp_io::read_bytes(&mut stream);
    if buffer.len() == 0 {
      break;
    }

    tcp_io::write_string(&stream, &buffer);
  }

  stream
}
