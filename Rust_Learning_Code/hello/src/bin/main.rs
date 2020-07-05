/*
 * Learning Rust: web_server
 * 2020/7/4
 * hustccc
 * Manjaro
 */
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;
fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        //println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        })
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    /*
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();
        //let contents = fs::read_to_string("Report_test.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else{
        //other requestions
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}",status_line,contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();       
    }*/
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
    
    
    
}

