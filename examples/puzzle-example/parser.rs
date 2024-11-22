use advent_of_code::puzzles::puzzle_error::PuzzleError;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[String]) -> Result<Vec<u32>, PuzzleError> {
        lines.iter().map(|line| Self::parse_line(line)).collect()
    }

    fn parse_line(line: &str) -> Result<u32, PuzzleError> {
        // Try to convert line to u32 number
        let number = line.parse::<u32>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to convert '{line}' to u32 with error '{err}'"
            ))
        })?;

        Ok(number)
    }
}
