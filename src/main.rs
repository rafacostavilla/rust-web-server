use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
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
    
    let success_status_line = "GET / HTTP/1.1";
    let mut file_path = "404.html";

    let mut status_line = "HTTP/1.1 404 NOT FOUND";

    if request_line == success_status_line{
        status_line = "HTTP/1.1 200 OK";
        file_path = "hello.html";
    }
    
    let content = fs::read_to_string(file_path).unwrap();
    let content_length = content.len(); 
        
    let response = 
        format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
            
    stream.write_all(response.as_bytes()).unwrap();
    
}