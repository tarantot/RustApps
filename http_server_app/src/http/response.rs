// The commented-out lines suggest that additional functionality related to formatting 
// and TCP stream handling was either once included or anticipated for use in the future.
// This shows forward-thinking and preparation for possible expansion of the code.

// use std::{fmt::Formatter, io::{Result as IoResult, Write}};
use std::io::{Result as IoResult, Write};
// use std::net::TcpStream;
// use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::HashMap;
use super::StatusCode;

#[derive(Debug)]

// Response structure
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    headers: HashMap<String, String>,
    version: String,
}

// Constructor method to create a new `Response` instance.
// Takes a `StatusCode` and an optional `String` for the body.
impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { 
            status_code, 
            body, 
            headers: HashMap::new(),
            version, 
        }
    }

    // Add a header to the response
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    // Send the HTTP response to any writable stream.
    // It abstracts over any type that implements the `Write` trait, allowing flexibility.
    // `IoResult<()>` indicates that the function returns an I/O operation result.
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        // The `write!` macro writes formatted data to the `stream`.
        // The HTTP response format includes the status code, reason phrase, and body.
        write!(
            stream,
            "{} {} {}\r\n",  // Use dynamic HTTP version instead of hardcoding "HTTP/1.1"
            self.version,     // Reference the version from the request
            self.status_code,
            self.status_code.reason_phrase()
        )?;

        // Write headers
        for (key, value) in &self.headers {
            write!(stream, "{}: {}\r\n", key, value)?;
        }

        // End headers and write body
        write!(stream, "\r\n{}", body)
    }
}

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//     }
// }