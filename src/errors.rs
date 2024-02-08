use std::fmt;

pub struct PathNotFoundError;

impl fmt::Display for PathNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Specified path or file is not found.")
    }
}