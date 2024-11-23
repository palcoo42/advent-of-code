use std::{thread::sleep, time::Duration};

use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use crate::parser::Parser;

pub struct Solver {
    numbers: Vec<u32>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            numbers: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Puzzle ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.numbers = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        // Simulate long running task to see progress bar
        sleep(Duration::from_secs(3));
        Ok(self.numbers.iter().sum::<u32>().to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(self.numbers.iter().product::<u32>().to_string())
    }
}
