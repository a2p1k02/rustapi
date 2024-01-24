use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use serde_json::Value;

fn handle_connection(mut stream: TcpStream, paths: HashMap<&str, Value>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let content_type = "application/json";

    let path = request_line[4..request_line.len() - 9].to_string().clone();

    if !paths.get(path.as_str()).is_none() {
        let content = paths.get(path.as_str()).unwrap();
        let response =
            format!("HTTP/1.1 200 OK\r\nContent-Type:{content_type}\r\n\r\n{content}", );
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response =
            format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type:{content_type}\r\n\r\n{}",
                    serde_json::json!({"status": 404, "message": "page not found"})
            );
        stream.write_all(response.as_bytes()).unwrap();
    }

}

pub fn start(address: &str, paths: HashMap<&str, Value>) {
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, paths.clone());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_api() {
        let mut paths = HashMap::new();

        paths.insert("/", serde_json::json!({"status": 200, "message": "hello world"}));
        paths.insert("/test", serde_json::json!({"status": 200, "message": "hello from /test page"}));

        assert_eq!(paths.len(), 2);

        //start("127.0.0.1:5000", paths);
    }
}
