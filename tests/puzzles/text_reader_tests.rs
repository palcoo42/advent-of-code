use std::path::Path;

use advent_of_code::{
    env::project::Project,
    puzzles::{puzzle_error::PuzzleError, reader::text_reader::TextReader},
};

#[test]
fn test_text_reader() {
    let input_file = Project::new().resource_test_file("input.txt");
    let reader = TextReader::new(&input_file);

    let result = reader.read_lines();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!["1", "2", "3", "4", "5"]);
}

#[test]
fn test_text_reader_not_existing_file() {
    let input_file = &Path::new("not-existing");
    let reader = TextReader::new(input_file);

    let result = reader.read_lines();

    assert!(result.is_err());
    assert!(
        matches!(result, Err(PuzzleError::OpenFileError(_, _)),),
        "result: {:?}",
        result
    )
}

#[test]
fn test_text_reader_empty_file() {
    let input_file = Project::new().resource_test_file("empty_file.txt");
    let reader = TextReader::new(&input_file);

    let result = reader.read_lines();

    assert!(result.is_err());
    assert!(
        matches!(result, Err(PuzzleError::EmptyFileError(_))),
        "result: {:?}",
        result
    );
}
