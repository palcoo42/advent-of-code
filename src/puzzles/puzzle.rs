use std::{path::Path, time::Instant};

use crate::puzzles::solution_progress_bar_thread::SolutionProgressBarThread;

use super::{
    puzzle_error::PuzzleError, puzzle_solver::PuzzleSolver, reader::text_reader::TextReader,
};

/// Definition of the result of the puzzle
pub type PuzzleResult = Result<(), PuzzleError>;

/// Definition of the result of the solution (part 1, part 2)
pub type SolutionResult = Result<String, PuzzleError>;

/// Prefixes for console output
const TIME_PREFIX: &str = "----:---"; // seconds:miliseconds
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

        let timer = Instant::now();

        // Read input file if present
        self.read_input_file(&timer)?;

        // Solve puzzle part 1
        self.solve_part_1(&timer)?;

        // Solve puzzle part 2
        self.solve_part_2(&timer)?;

        // If we get here everything is fine
        Ok(())
    }

    fn print_result(timer: &Instant, prefix: &str, result: &str) {
        println!(
            "{:04}:{:03} {} {}",
            timer.elapsed().as_secs(),
            timer.elapsed().subsec_millis(),
            prefix,
            result
        );
    }

    fn read_input_file(&mut self, timer: &Instant) -> PuzzleResult {
        let result;

        {
            let prefix = format!("{} {}", TIME_PREFIX, READ_INPUT_FILE_PREFIX);
            let mut progress = SolutionProgressBarThread::new(&prefix);
            progress.run();

            result = match &self.reader {
                Some(reader) => {
                    // Read lines from input file
                    let lines = reader.read_lines()?;
                    let lines = lines.iter().map(|line| line.as_str()).collect::<Vec<_>>();

                    // Parse input file and report possible error
                    self.solver.parse_input_file(&lines)?;

                    format!("Done [{}]", reader.get_file_path().to_string_lossy())
                }
                None => String::from("No input file"),
            };
        }

        Self::print_result(timer, READ_INPUT_FILE_PREFIX, &result);
        Ok(())
    }

    fn solve_part_1(&self, timer: &Instant) -> PuzzleResult {
        let result;

        {
            let prefix = format!("{} {}", TIME_PREFIX, PART_1_PREFIX);
            let mut progress = SolutionProgressBarThread::new(&prefix);
            progress.run();

            result = self.solver.part_1()?;
        }

        Self::print_result(timer, PART_1_PREFIX, &result);
        Ok(())
    }

    fn solve_part_2(&self, timer: &Instant) -> PuzzleResult {
        let result;

        {
            let prefix = format!("{} {}", TIME_PREFIX, PART_2_PREFIX);
            let mut progress = SolutionProgressBarThread::new(&prefix);
            progress.run();

            result = self.solver.part_2()?;
        }

        Self::print_result(timer, PART_2_PREFIX, &result);
        Ok(())
    }
}
