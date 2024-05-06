use rust_web_server::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    // read first line then iterate over rest of the lines in buf_reader
    let mut first_line = String::new();
    buf_reader.read_line(&mut first_line).unwrap();
    if first_line.starts_with("GET") {
        return handle_get_request(stream, first_line);
    }
    for line in buf_reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!("HTTP/1.1 200 OK \r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
fn handle_get_request(mut stream: TcpStream, first_line: String) {
    let resource = first_line.split_whitespace().collect::<Vec<&str>>()[1];
    let file_content = read_from_rile(&resource[..]);
    match file_content {
        Ok(content) => {
            let length = content.len();
            let response = format!(
                "HTTP/1.1 200 OK \r\nContent-Length: {length}\r\n\r\n{content}",
                length = length,
                content = content
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            let response = format!("HTTP/1.1 404 NOT FOUND \r\n\r\n{}", e);
            stream.write_all(response.as_bytes()).unwrap();
        }
   }
    /* stream
        .write_all(
            format!(
                "HTTP/1.1 200 OK \r\n\r\n Hello from backend {}",
                file_content
            )
            .as_bytes(),
        )
        .unwrap();*/
}
fn read_from_rile(file_name: &str) -> Result<String, std::io::Error> {
    let file_name = format!("./public/{}.txt", file_name);
    println!("file_name: {}", file_name);
    fs::read_to_string(file_name)
}
