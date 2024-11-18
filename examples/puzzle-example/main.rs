mod parser;
mod solver;

use advent_of_code::{
    env::project::Project,
    puzzles::puzzle::{Puzzle, PuzzleResult},
};
use solver::Solver;

fn main() -> PuzzleResult {
    let input_file = Project::new().project_file(&["examples", "resources"], "numbers.txt");
    let mut puzzle = Puzzle::<Solver>::new_with_reader(&input_file);
    puzzle.solve()
}
