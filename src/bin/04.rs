advent_of_code::solution!(4);
use anyhow::Result;

pub fn part_one(input: &str) -> Result<usize> {
    let _matrix = parse(input);

    unimplemented!()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let matrix = input.lines().map(|line| line.chars().collect()).collect();
    dbg!(matrix)
}

pub fn part_two(_input: &str) -> Result<usize> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 18);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 0);

        Ok(())
    }
}
