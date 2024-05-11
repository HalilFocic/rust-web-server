use rust_web_server::ThreadPool;
use serde::Deserialize;
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
        "POST" => match read_body_from_request(&mut buf_reader) {
            Ok(text_content) => return handle_post_request(stream, split_first_line, text_content),
            Err(_) => return handle_unsupported_request(stream),
        },
        "PATCH" => match read_body_from_request(&mut buf_reader) {
            Ok(text_content) => {
                return handle_patch_request(stream, split_first_line, text_content)
            }
            Err(_) => return handle_unsupported_request(stream),
        },
        "DELETE" => return handle_delete_request(stream, split_first_line),
        _ => return handle_unsupported_request(stream),
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
fn handle_post_request(mut stream: TcpStream, first_line: Vec<&str>, text_content: String) {
    let path = format!("./public{}", first_line[1]);
    let mut resource: File;

    if let Ok(file) = OpenOptions::new().append(true).open(&path) {
        resource = file;
    } else {
        resource = File::create(&path).unwrap();
    }
    // write the string text_content to file
    resource.write_all(text_content.as_bytes()).unwrap();
    let save_successful = "Save was sucessfull";
    let content = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        save_successful.len(),
        save_successful
    );
    return stream.write_all(&content.as_bytes()).unwrap();
}
fn handle_patch_request(mut stream: TcpStream, first_line: Vec<&str>, text_content: String) {
    let path = format!("./public{}", first_line[1]);
    let mut resource: File;

    if let Ok(file) = OpenOptions::new().write(true).truncate(true).open(&path) {
        resource = file;
    } else {
        resource = File::create(&path).unwrap();
    }
    resource.write_all(text_content.as_bytes()).unwrap();
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
fn handle_unsupported_request(mut stream: TcpStream) {
    let save_successful = "Unsupported request";
    let content = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        save_successful.len(),
        save_successful
    );
    return stream.write_all(content.as_bytes()).unwrap();
}

fn read_from_rile(file_name: &str) -> Result<String, std::io::Error> {
    let file_name = format!("./public/{}", file_name);
    println!("file_name: {}", file_name);
    fs::read_to_string(file_name)
}
#[derive(Deserialize, Debug)]
struct RequestBody {
    content: String,
}
fn read_body_from_request(
    buf_reader: &mut BufReader<&mut TcpStream>,
) -> Result<String, serde_json::Error> {
    let mut content_length: usize = 0;

    loop {
        let line = buf_reader.lines().next().unwrap().unwrap();
        if line.starts_with("Content-Length:") {
            let split_line = line.split(":").collect::<Vec<&str>>();
            content_length = split_line[1].trim().parse::<usize>().unwrap();
        }
        if line.trim().is_empty() {
            break;
        }
    }
    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body).unwrap();
    // convert the body to string
    let body = String::from_utf8(body).unwrap();
    // parse the body to a struct or return empty string if there is an error
    let body: RequestBody = serde_json::from_str(&body)?;
    return Ok(body.content);
}
