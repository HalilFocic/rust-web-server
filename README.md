#  Multithreaded Web Server in Rust

Web server written in Rust that uses ThreadPools to handle multiple requests. Supports GET, POST, PATCH and DELETE methods. \
For example hitting the route /file.txt will resolve in following:
- GET: Retrieves the text content of the file at /public/file.txt
- POST: Appends text content to file or creates the file if it wasn't present.
- PATCH: Replaced text content in a file. Throws error if file is not present.
- DELETE: Deletes the file or throws error if it doesn't exist.

1. Clone repository
```
$ git clone https://github.com/HalilFocic/rust-web-server.git
```
2. Run the project
```
cargo run 
```

Future plans for the project:
- Add file tree view to index page ( / )
- Add ability to write and preview markdown
  
