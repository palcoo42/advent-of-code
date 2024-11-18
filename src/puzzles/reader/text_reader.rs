use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::puzzles::puzzle_error::PuzzleError;

/// Preallocated capacity of vector where lines are stored
const LINES_CAPACITY: usize = 1024;

/// Reads input text file
pub struct TextReader {
    path: PathBuf,
}

impl TextReader {
    /// Creates a new instance of text reader
    ///
    /// # Arguments
    ///
    /// _path_ - Path to the text file
    ///
    /// # Returns
    ///
    /// New instance of TextReader
    ///
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    /// Get path to the file which is analyzed
    pub fn get_file_path(&self) -> &Path {
        &self.path
    }

    /// Get path to the file as string representation
    pub fn get_file_path_as_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn read_lines(&self) -> Result<Vec<String>, PuzzleError> {
        // Open file for text reading
        let file = File::open(&self.path).map_err(|err| {
            PuzzleError::OpenFileError(self.path.to_string_lossy().to_string(), err)
        })?;

        // Create buffered reader for efficient reading
        let reader = BufReader::new(file);

        let mut lines = Vec::with_capacity(LINES_CAPACITY);

        // Read all lines from the file
        for read_result in reader.lines() {
            // Check read result
            match read_result {
                Ok(line) => lines.push(line),
                Err(err) => {
                    return Err(PuzzleError::ReadFileError(
                        self.path.to_string_lossy().to_string(),
                        err,
                    ))
                }
            }
        }

        // If there are no lines at all file is probably invalid
        if lines.is_empty() {
            return Err(PuzzleError::EmptyFileError(
                self.path.to_string_lossy().to_string(),
            ));
        }

        Ok(lines)
    }
}
