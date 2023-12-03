use std::error::Error;
use std::fs;
use std::str;

pub fn read_file(path: &str) -> Result<Box<str>, Box<dyn Error + 'static>> {
    match fs::read_to_string(path) {
        Ok(string) => Ok(Box::from(string)),
        Err(err) => Err(Box::new(err)),
    }
}
