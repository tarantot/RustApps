use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // TCP connections

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // data size to read
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // println!("Received a request: {:?}", String::from_utf8_lossy(&buffer)); // debug format
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer)); // converts bytes to a string, including invalid characters

                            // Solution with the extra conversion to string with allocating.
                            // match Request::try_from(&buffer[..]) {
                            //     Ok(request) => {
                            //         dbg!(request);
                            //         let response = Response::new(StatusCode::OK,
                            //         Some("<h1> IT WORKS !!! </h1>".to_string()),
                            //         );
                            //         response.send(&mut stream);
                            //         // write!(stream, "HTTP/1.1 404 Not Fount\r\n\r\n");
                            //         write!(stream, "{}", response);
                            //     }
                            //     Err(e) => {
                            //         println!("Failed to parse request: {}", e);
                            //         Response::new(StatusCode::BadRequest, None).send(&mut stream);
                            //     }
                            // }
                            
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                // let res: &Result<Request, _> = &buffer[..].try.into(),
                                // Err(e) => println!("Failed to parse request: {}", e),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}