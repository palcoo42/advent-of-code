use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_error::PuzzleError,
    puzzle_solver::PuzzleSolver,
};

/// Implements puzzle solver for test purposes
pub struct TestPuzzleSolver {
    content: Vec<u32>,
}

impl PuzzleSolver for TestPuzzleSolver {
    fn new() -> Self {
        TestPuzzleSolver {
            content: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Test Puzzle ---"
    }

    fn parse_input_file(&mut self, lines: &[String]) -> PuzzleResult {
        let mut content = Vec::new();

        for line in lines {
            let number = line.parse::<u32>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert '{line}' to u32 with error '{err}'"
                ))
            })?;

            content.push(number);
        }

        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        Ok(self.content.iter().sum::<u32>().to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(self.content.iter().product::<u32>().to_string())
    }
}
