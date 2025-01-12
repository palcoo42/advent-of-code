use super::puzzle::{PuzzleResult, SolutionResult};

/// Defines requirements for real puzzle solver which can be used to solve Advent of Code puzzles.
pub trait PuzzleSolver {
    /// Creates a new instance of puzzle solver
    ///
    /// # Returns
    ///
    /// New instance of puzzle solver
    fn new() -> Self;

    /// Get puzzle description
    ///
    /// # Returns
    ///
    /// String representation of puzzle description
    fn get_description(&self) -> &str;

    /// Parse content of the input file
    /// This method is called only if [Puzzle] is created with reader
    ///
    /// # Arguments
    ///
    /// _lines_ - Lines read from input file
    ///
    /// # Returns
    ///
    /// Empty result on success, error on failure
    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult;

    /// Solve part 1 of the puzzle
    ///
    /// # Returns
    ///
    /// String representation of the solution on success, error on failure
    fn part_1(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }

    /// Solve part 2 of the puzzle
    ///
    /// # Returns
    ///
    /// String representation of the solution on success, error on failure
    fn part_2(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }
}
