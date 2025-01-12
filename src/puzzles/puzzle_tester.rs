use std::{fs::File, io::BufRead, io::BufReader, sync::Mutex};

use crate::env::project::Project;

use super::puzzle_solver::PuzzleSolver;

pub struct PuzzleTester<T>
where
    T: PuzzleSolver,
{
    solver: Mutex<T>,
    solution_1: String,
    solution_2: String,
}

impl<T> PuzzleTester<T>
where
    T: PuzzleSolver,
{
    pub fn new(solution_1: &str, solution_2: &str) -> Self {
        let solver = Self::create_solver();
        Self {
            solver,
            solution_1: String::from(solution_1),
            solution_2: String::from(solution_2),
        }
    }

    /// Create a solver instance which can be used to validate parts algorithms.
    ///
    /// # Returns
    ///
    /// Mutex to solver because unit tests are executed in a different threads
    fn create_solver() -> Mutex<T> {
        // Read input file
        let input_file = Project::new().resource_file("input.txt");
        let file = File::open(&input_file)
            .unwrap_or_else(|err| panic!("Failed to open file with an error '{}'", err));
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap_or_else(|err| panic!("Failed to unwrap lines with an error '{}'", err));

        let lines: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();

        // Prepare solver
        let mut solver = T::new();

        // Parse input file
        solver
            .parse_input_file(&lines)
            .unwrap_or_else(|err| panic!("Failed to parse input file with error '{}'", err));

        Mutex::new(solver)
    }

    pub fn test_part_1(&self) {
        let result;

        // Solve the puzzle inside a scope so that guard is released automatically avoiding a panic in the thread.
        {
            let locked_solver = self.solver.lock().expect("Failed to unwrap 'Solver Mutex'");
            result = locked_solver.part_1();
        }

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), self.solution_1);
    }

    pub fn test_part_2(&self) {
        let result;

        // Solve the puzzle inside a scope so that guard is released automatically avoiding a panic in the thread.
        {
            let locked_solver = self.solver.lock().expect("Failed to unwrap 'Solver Mutex'");
            result = locked_solver.part_2();
        }

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), self.solution_2);
    }
}
