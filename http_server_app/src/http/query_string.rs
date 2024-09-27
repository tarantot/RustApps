// Importing the HashMap from the standard library for storing key-value pairs.
use std::collections::HashMap;

#[derive(Debug)]

// Define a public struct QueryString with a lifetime parameter 'buf.
// The struct contains a HashMap where keys are string slices and values are of type Value
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]

// Enum that represents the possible values of query string parameters.
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    // Method to retrieve a value by key. Returns an Option<&Value> (Some if the key exists, None otherwise).
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// Example of query string: a=1&b=2&c&d=&e===&d=7&d=abc

// Implementing the From trait to allow conversion from a &str to QueryString.
impl<'buf> From<&'buf str> for QueryString<'buf> {  
    fn from(s: &'buf str) -> Self {  
        let mut data = HashMap::new();  // Initialize an empty HashMap to store the key-value pairs.
        
        // Split the input string by '&' to handle each key-value pair separately.
        for sub_str in s.split('&') { 
            let mut key = sub_str;  
            let mut val = "";  
            
            // If an '=' is found, we split the substring into key and value.
            if let Some(i) = sub_str.find('=') { 
                key = &sub_str[..i];  
                val = &sub_str[i + 1..];  
            }

            // Insert the key into the HashMap or update the value if the key already exists.
            data.entry(key) 
                .and_modify(|existing: &mut Value| match existing {  // If the key exists, modify its value.
                    // If it was a Single value, convert it to Multiple by adding the previous and current values to a vector.
                    Value::Single(prev_val) => { 
                        *existing = Value::Multiple(vec![prev_val, val]);  
                    }
                    // If it's already Multiple, just add the new value to the vector.
                    Value::Multiple(vec) => vec.push(val),
                })
                // If the key doesn't exist yet, insert it with a Single value.
                .or_insert(Value::Single(val));
        }

        QueryString { data }  // Return the populated QueryString.
    }
}