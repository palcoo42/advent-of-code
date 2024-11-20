use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

pub struct Solver {}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {}
    }

    fn get_description(&self) -> &str {
        "--- Day 02: Bathroom Security ---"
    }

    fn parse_input_file(&mut self, lines: Vec<String>) -> PuzzleResult {
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }
}
