use std::path::Path;

use crate::puzzles::solution_progress_bar_thread::SolutionProgressBarThread;

use super::{
    puzzle_error::PuzzleError, puzzle_solver::PuzzleSolver, reader::text_reader::TextReader,
};

/// Definition of the result of the puzzle
pub type PuzzleResult = Result<(), PuzzleError>;

/// Definition of the result of the solution (part 1, part 2)
pub type SolutionResult = Result<String, PuzzleError>;

/// Prefixes for console output
const READ_INPUT_FILE_PREFIX: &str = "=> Reader:";
const PART_1_PREFIX: &str = "=> Part 1:";
const PART_2_PREFIX: &str = "=> Part 2:";

#[derive(Default)]
pub struct Puzzle<T> {
    /// Text file reader
    reader: Option<TextReader>,

    /// Puzzle solver
    solver: T,
}

impl<T> Puzzle<T>
where
    T: PuzzleSolver,
{
    /// Creates new instance of the Puzzle without file reader, i.e. there is no associated file
    /// with input data for this puzzle.
    ///
    /// # Returns
    ///
    /// Instance of Puzzle specialized for concrete PuzzleSolver
    pub fn new() -> Self {
        Self {
            reader: None,
            solver: T::new(),
        }
    }

    /// Creates new instance of the Puzzle with a file reader, i.e. there is an associated file
    /// with input data for this puzzle.
    ///
    /// # Arguments
    ///
    /// _path_ - Path to the input file with puzzle content
    ///
    /// # Returns
    ///
    /// Instance of Puzzle specialized for concrete PuzzleSolver
    pub fn new_with_reader(path: &Path) -> Self {
        Self {
            reader: Some(TextReader::new(path)),
            solver: T::new(),
        }
    }

    /// Solve puzzle - there are multiple steps which are done in a sequence:
    ///
    /// - Read input file if requested
    /// - Solve puzzle part 1
    /// - Solve puzzle part 2
    ///
    /// # Returns
    ///
    /// Successful result or specific error occurred during the solving of the puzzle
    pub fn solve(&mut self) -> PuzzleResult {
        println!("{}", self.solver.get_description());
        println!();

        // Read input file if present
        self.read_input_file()?;

        // Solve puzzle part 1
        let result_part_1 = self.solve_part_1()?;
        println!("{} {}", PART_1_PREFIX, result_part_1);

        // Solve puzzle part 2
        let result_part_2 = self.solve_part_2()?;
        println!("{} {}", PART_2_PREFIX, result_part_2);

        // If we get here everything is fine
        Ok(())
    }

    fn read_input_file(&mut self) -> PuzzleResult {
        match &self.reader {
            Some(reader) => {
                println!(
                    "{} Reading input file '{}'",
                    READ_INPUT_FILE_PREFIX,
                    reader.get_file_path_as_string()
                );

                // Read lines from input file
                let lines = reader.read_lines()?;

                // Parse input file and report possible error
                self.solver.parse_input_file(&lines[..])?;
            }
            None => println!("{} Nothing to do", READ_INPUT_FILE_PREFIX),
        }

        Ok(())
    }

    fn solve_part_1(&self) -> SolutionResult {
        let mut progress = SolutionProgressBarThread::new(PART_1_PREFIX);
        progress.run();

        self.solver.part_1()
    }

    fn solve_part_2(&self) -> SolutionResult {
        let mut progress = SolutionProgressBarThread::new(PART_2_PREFIX);
        progress.run();

        self.solver.part_2()
    }
}
