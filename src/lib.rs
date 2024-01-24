use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use chrono::{Local};
use serde_json::Value;

fn print_routes(paths: HashMap<&str, Value>) {
    print!("Available routes: ");
    for (k, _) in &paths {
        print!("{k} ");
    }
    println!();
}

fn handle_connection(mut stream: TcpStream, paths: HashMap<&str, Value>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let content_type = "application/json";

    let path = request_line[4..request_line.len() - 9].to_string().clone();

    if !paths.get(path.as_str()).is_none() {
        let content = paths.get(path.as_str()).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type:{content_type}\r\n\r\n{content}", );
        println!("[{}] {} {} - HTTP/1.1 200 OK",
                 Local::now().format("%c"), stream.peer_addr().unwrap().to_string(), path.as_str());
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response =
            format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type:{content_type}\r\n\r\n{}",
                    serde_json::json!({"status": 404, "message": "page not found"})
            );
        println!("[{}] {} {} - HTTP/1.1 404 NOT FOUND",
                 Local::now().format("%c"), stream.peer_addr().unwrap().to_string(), path.as_str());
        stream.write_all(response.as_bytes()).unwrap();
    }

}

pub fn start(address: &str, paths: HashMap<&str, Value>) {
    println!("[{}] Connecting...", Local::now().format("%c"));
    let listener = TcpListener::bind(address).unwrap();
    println!("[{}] Connected!", Local::now().format("%c"));
    println!("Server available on {address}");

    print_routes(paths.clone());

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
