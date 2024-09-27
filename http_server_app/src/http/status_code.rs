use std::fmt::{Display, Formatter, Result as FmtResult};

// - `Copy` for values of the enum can be duplicated by simply copying bits.
// - `Clone` allows explicit duplication of the enum instances.
// - `Debug` enables formatting the enum variants for debugging output.
#[derive(Copy, Clone, Debug)]

// Identifying status codes
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

// To return the associated human-readable reason for each status code.
impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

// Implementing the `Display` trait for `StatusCode`.
// The `Display` trait allows to define how the `StatusCode` is formatted when printed.
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}