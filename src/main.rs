use rust_web_server::ThreadPool;
use std::fs::OpenOptions;
use std::{
    fs::{self, File},
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

    let mut first_line = String::new();
    buf_reader.read_line(&mut first_line).unwrap();
    let split_first_line = first_line.split_whitespace().collect::<Vec<&str>>();
    match split_first_line[0] {
        "GET" => return handle_get_request(stream, first_line),
        "POST" => return handle_post_request(stream, split_first_line),
        "PATCH" => return handle_patch_request(stream, split_first_line),
        "DELETE" => return handle_delete_request(stream, split_first_line),
        _ => return handle_unsupported_request(stream, first_line),
    }
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
        Err(_) => {
            let response = "HTTP/1.1 404 NOT FOUND \r\n\r\n Seems like this file doesn't exist";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
fn handle_post_request(mut stream: TcpStream, first_line: Vec<&str>) {
    let path = format!("./public{}.txt", first_line[1]);
    let mut resource: File;

    if let Ok(file) = OpenOptions::new().append(true).open(&path) {
        resource = file;
    } else {
        println!("File not found");
        resource = File::create(&path).unwrap();
    }
    resource.write_all("Appendam 123\n".as_bytes()).unwrap();
    let save_successful = "Save was sucessfull";
    let content = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        save_successful.len(),
        save_successful
    );
    return stream.write_all(content.as_bytes()).unwrap();
}
fn handle_patch_request(mut stream: TcpStream, first_line: Vec<&str>) {
    let path = format!("./public{}.txt", first_line[1]);
    let mut resource: File;

    if let Ok(file) = OpenOptions::new().write(true).truncate(true).open(&path) {
        resource = file;
    } else {
        println!("File not found");
        resource = File::create(&path).unwrap();
    }
    resource.write_all("Patch 123\n".as_bytes()).unwrap();
    let save_successful = "Patch was sucessfull";
    let content = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        save_successful.len(),
        save_successful
    );
    return stream.write_all(content.as_bytes()).unwrap();
}
fn handle_delete_request(mut stream: TcpStream, first_line: Vec<&str>) {
    let path = format!("./public{}.txt", first_line[1]);
    if let Ok(_) = fs::remove_file(&path) {
        let save_successful = "Delete was sucessfull";
        let content = format!(
            "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
            save_successful.len(),
            save_successful
        );
        return stream.write_all(content.as_bytes()).unwrap();
    } else {
        let delete_not_successful = "Unable to delete the file";
        let content = format!(
            "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
            delete_not_successful.len(),
            delete_not_successful
        );
        return stream.write_all(content.as_bytes()).unwrap();
    }
}
fn handle_unsupported_request(mut stream: TcpStream, first_line: String) {
    let save_successful = "Unsupported request";
    let content = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        save_successful.len(),
        save_successful
    );
    return stream.write_all(content.as_bytes()).unwrap();
}

fn read_from_rile(file_name: &str) -> Result<String, std::io::Error> {
    let file_name = format!("./public/{}.txt", file_name);
    println!("file_name: {}", file_name);
    fs::read_to_string(file_name)
}
