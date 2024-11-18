use advent_of_code::{env::project::Project, puzzles::puzzle::Puzzle};

use super::test_puzzle_solver::TestPuzzleSolver;

#[test]
fn test_puzzle_no_reader() {
    // Create test puzzle
    let mut puzzle: Puzzle<TestPuzzleSolver> = Puzzle::new();

    // Solve the puzzle and check the results
    let result = puzzle.solve();

    assert!(result.is_ok(), "Puzzle error: {:?}", result);
}

#[test]
fn test_puzzle_with_reader() {
    // Create test puzzle
    let input_file = Project::new().resource_test_file("input.txt");

    let mut puzzle: Puzzle<TestPuzzleSolver> = Puzzle::new_with_reader(&input_file);

    // Solve the puzzle and check the results
    let result = puzzle.solve();

    assert!(result.is_ok(), "Puzzle error: {:?}", result);
}
