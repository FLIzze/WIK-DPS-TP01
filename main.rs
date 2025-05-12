use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const HOST: &str = "0.0.0.0";
const PORT: &str = "8080";

fn main() {
    let address = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(&address).expect("Failed to start server");

    println!("Server running on http://{}", address);

    for connection in listener.incoming() {
        if let Ok(mut stream) = connection {
            handle_client(&mut stream);
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    println!("lets go");
    let mut buffer = [0; 1024];
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer);

        if request.starts_with("GET /ping ") {
            let headers = extract_headers(&request);
            let json_response = headers_to_json(&headers);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                json_response.len(),
                json_response
            );
            stream.write_all(response.as_bytes()).expect("Failed to write response");
        } else {
            let not_found = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(not_found.as_bytes()).expect("Failed to write response");
        }
    }
}

fn extract_headers(request: &str) -> HashMap<String, String> {
    request.lines()
        .skip(1) // Skip request line
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.splitn(2, ':');
            Some((parts.next()?.trim().to_string(), parts.next()?.trim().to_string()))
        })
    .collect()
}

fn headers_to_json(headers: &HashMap<String, String>) -> String {
    let mut json = String::from("{");
    let mut first = true;

    for (key, value) in headers {
        if !first {
            json.push(',');
        }
        json.push_str(&format!("\"{}\":\"{}\"", key, value));
        first = false;
    }

    json.push('}');
    json
}
