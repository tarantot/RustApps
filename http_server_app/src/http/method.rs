use std::str::FromStr;

// Derive the Debug trait for the Method enum to allow easy printing for debugging purposes.
// The Method enum represents different HTTP methods commonly used in web communication.
#[derive(Debug)]

// Defining an enum `Method` that represents HTTP methods, which are common in web development.
// Each variant corresponds to a different HTTP request method, giving this enum a fixed set of values.
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

// Implementing the FromStr trait for the Method enum, allowing strings to be parsed into Method variants.
// This implementation will enable the conversion of string literals into their corresponding Method enum values.
impl FromStr for Method {
    type Err = MethodError;

    // Defining the from_str function that converts a string slice (&str) into a Method enum variant.
    // This function returns a Result, which is either Ok(Method) or Err(MethodError).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Using a match expression to map string values to their corresponding enum variants.
        // If the string matches a known HTTP method, the corresponding variant is returned.
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

// Define a custom error type to represent the failure of parsing a string into a Method.
// This struct does not yet contain any additional data or functionality, but serves as a type for error handling.
pub struct MethodError;