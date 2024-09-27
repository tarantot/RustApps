#![allow(dead_code)]

use server::Server;
// use http::Request;
// use http::Method;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() { 
    /* 
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];
    let string_borrow: &str = &string;
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal); */

    // REQUESTS 
    /*
    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE(0);
    let post = Method::POST;
    let put = Method::PUT; */
    
    // Path to the directory containing the Cargo.toml file and creating a string 
    // that includes the path to a `public` directory relative to the project directory.
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    
    println!("public path: {}", public_path);
    
    // Creating a new server instance that listens on `127.0.0.1:8080`.
    let server = Server::new("127.0.0.1:8080".to_string());
    
    // Start running the server and passes in the WebsiteHandler
    server.run(WebsiteHandler::new(public_path));
}