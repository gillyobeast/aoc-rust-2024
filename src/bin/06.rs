advent_of_code::solution!(6);
use advent_of_code::table::Table;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<usize> {
    let matrix: Table = Table::parse(input);
    println!("{:?}", matrix);
    println!("{:?}", matrix.clone().rotate());
    println!("{:?}", matrix.rotate().rotate());
    unimplemented!()
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
        assert_eq!(result, 41);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 0);

        Ok(())
    }
}
