use std::io;
use regex;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    RegexError(regex::Error),
}

#[derive(Debug)]
pub struct WalkrError {
    error: Error
}

impl WalkrError {
    pub fn new(e: Error) -> WalkrError {
        WalkrError{
            error: e
        }
    }
}

impl std::error::Error for WalkrError {
    fn description(&self) -> &str {
        "Walkr Error"
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let s = self.clone();
        match &s.error {
            Error::IOError(e) => {
                return Some(e);
            },
            Error::RegexError(e) => {
                return Some(e);                
            }
        };
    }
}

impl fmt::Display for WalkrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WalkrError is here!")
    }
}

