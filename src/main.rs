use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream},  time::Duration};
use std::thread;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        thread::spawn(||
            handle_connection(stream)
        );
    }
}

// This function consumes stream
fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
    
    let (status_line,filepath) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK","hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK","hello.html")},
        _ => ("HTTP/1.1 404 NOT FOUND","404.html"),   
    };
    
    let content = fs::read_to_string(filepath).unwrap();
    let content_length = content.len(); 
        
    let response = 
        format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
            
    stream.write_all(response.as_bytes()).unwrap();
    
}