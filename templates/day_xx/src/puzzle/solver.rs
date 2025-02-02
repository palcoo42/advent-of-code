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
        Ok(String::from(""))
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from(""))
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "Not solved";
    const SOLUTION_2: &str = "Not solved";

    fn get_tester() -> &'static PuzzleTester<Solver> {
        static TESTER: LazyLock<PuzzleTester<Solver>> =
            LazyLock::new(|| PuzzleTester::new(SOLUTION_1, SOLUTION_2));

        &TESTER
    }

    // #[test]
    // fn test_part_1() {
    //     get_tester().test_part_1();
    // }

    // #[test]
    // fn test_part_2() {
    //     get_tester().test_part_2();
    // }
}
