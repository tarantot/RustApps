// use crate::http::{query_string, request};
use super::method::{Method, MethodError};
use super::QueryString;
// use std::collections::HashMap;
use std::convert::TryFrom;
use std::collections::HashMap;
// use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]

// Request struct with lifetime 'buf, containing path, query_string, and method
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, 
    method: Method,
    headers: HashMap<String, String>, 
}

// Implementation block for Request
impl<'buf> Request<'buf> {
    // Request
    // pub fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

// Implementing TryFrom trait to allow converting from a byte array to a Request type
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // type Error = String;
    type Error = ParseError;

    // Implementing the core conversion logic from a byte array to Request
    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(_) => return Err(e),
        // }

        // Parsing the incoming byte array as a UTF-8 string, propagating errors if invalid encoding is found
        
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        let request = str::from_utf8(buf)?;
        // request.encrypt();
        // buf.encrypt();

        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }
        let (method_line, headers) = request.split_once("\r\n").ok_or(ParseError::InvalidRequest)?;
        let (method, path_and_query) = method_line.split_once(" ").ok_or(ParseError::InvalidRequest)?;
        let (path, query_string) = match path_and_query.split_once('?') {
            Some((p, q)) => (p, Some(q)),
            None => (path_and_query, None),
            };

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" && protocol != "HTTP/2" {
            return Err(ParseError::InvalidProtocol);
        }

        // Parsing the HTTP method, propagating parsing errors
        let method: Method = method.parse()?;

        // Parse headers into HashMap
        let mut headers_map = HashMap::new();
        for line in headers.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                headers_map.insert(key.to_string(), value.to_string());
            }
        }

        let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }
        
        // Checking if there's a query string by searching for a '?', and splitting the path accordingly
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        // Returning a successfully parsed Request
        Ok(Request {
            path,
            query_string: query_string.map(QueryString::from),
            method: method.parse()?,  
            headers: headers_map,     
        });

        unimplemented!();
    }
}

// Trait to define encryption functionality for types
trait Encrypt {
    fn encrypt (&self) -> Self;
}

// Implementing Encrypt trait for String type
impl Encrypt for String {
    fn encrypt (&self) -> Self {
        unimplemented!()
    }
}

// Implementing Encrypt trait for byte slices
impl Encrypt for &[u8] {
    fn encrypt (&self) -> Self {
        unimplemented!()
    }
}

// Utility function to extract the next word from a request string, splitting by space or carriage return
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // unimplemented!()
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break;
    //     }
    // }
    
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

// Representing different types of possible errors
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// Messages for different errors
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

// Trait to convert MethodError into ParseError
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

// Trait from UTF8 for conversions that cannot fail
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// Display trait
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Debug trait
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// impl Error for ParseError {}