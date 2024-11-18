use std::{error::Error, fmt::Display};

/// Collection of errors which can arise during solving of the puzzle
#[derive(Debug)]
pub enum PuzzleError {
    OpenFileError(String, std::io::Error), // file name, io error
    ReadFileError(String, std::io::Error), // file name, io error
    EmptyFileError(String),                // file name
    InvalidContentError(String),           // error description
    GenericError(String),                  // error description
}

impl Error for PuzzleError {}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            PuzzleError::OpenFileError(file_name, err) => {
                format!("Open file error '{}' [{}]", file_name, err)
            }
            PuzzleError::ReadFileError(file_name, err) => {
                format!("Read file error '{}' [{}]", file_name, err)
            }
            PuzzleError::EmptyFileError(file_name) => {
                format!("Empty file error '{}'", file_name)
            }
            PuzzleError::InvalidContentError(err) => err.to_string(),
            PuzzleError::GenericError(err) => err.to_string(),
        };

        write!(f, "{}", msg)
    }
}
