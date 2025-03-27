use sha2::{Digest, Sha256};
use std::io::{stdin, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

fn main() {
    const CHUNK_SIZE: i32 = 50_000;

    let mut range_start = 0;
    let mut range_end = CHUNK_SIZE;

    let mut data: Vec<u8> = Vec::new();
    let mut length: usize = 0;

    let mut server_hash: String=String::new();
    println!("Paste server generated hash");
    stdin().read_line(&mut server_hash).expect("Failed to get user input for server generated hash");

    loop {
        data.extend_from_slice(&get_range_data(range_start, range_end,&mut length));
        println!("Content length: {}",length);

        if length as i32!=CHUNK_SIZE{
            break;
        }
        range_end += CHUNK_SIZE;
        range_start += CHUNK_SIZE;
    }
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_result = hasher.finalize();
    
    if format!("{:x}",hash_result)==server_hash.trim() {
        println!("Successful download")
    }
    else{
        println!("Incomplete download")
    }
}

fn get_range_data(range_start: i32, range_end: i32,length:&mut usize) -> Vec<u8> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    let request: String = format!(
        "GET / HTTP/1.1\r\nConnection: close\r\nRange: bytes={}-{}\r\n\r\n",
        range_start, range_end
    );

    stream
        .write_all(request.as_bytes())
        .expect("failed to send request");
    let mut response = BufReader::new(stream);
    let mut line = String::new();
    let mut body = Vec::new();
    loop {
        line.clear();
        response.read_line(&mut line).expect("error reading line");

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            break;
        } else if trimmed_line.starts_with("Content-Length: ") {
            if let Some(len) = trimmed_line.split(": ").nth(1) {
                *length = len.parse::<usize>().expect("");
            }
        }
    }
    response.read_to_end(&mut body).expect("error reading body");
    body
}
