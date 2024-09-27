use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

// Struct for handling website-related functionality
pub struct WebsiteHandler {
    public_path: String,
}

// Methods for the `WebsiteHandler` struct
impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    // Read a file from the specified file path.
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // fs::read_to_string(path).ok();

        // Match on the result of attempting to canonicalize the file path
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            // If canonicalization fails
            Err(_) => None,
        }
    }
}

// Processing requests and redirecting to pages
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::OK, Some("<h1> TEST </h1>".to_string()));
        match request.method() {
            // Handle GET requests
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            // For any other HTTP method, return a 404 NotFound response
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}