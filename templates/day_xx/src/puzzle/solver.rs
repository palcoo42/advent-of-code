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
        "TEMPLATE_SOLVER_DESCRIPTION"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }
}
