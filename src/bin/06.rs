advent_of_code::solution!(6);
use advent_of_code::table::Table;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<usize> {
    // first rotate the table so the cursor moves right through the row
    let table: Table = Table::parse(input).rotate();
    dbg!(table);
    // then recurse:
    // base case: the cursor does not meet a # => add spaces left in row to visited; break
    // recursive case: cursor does meet a # =>
    //      add spaces between cursor and # to visited;
    //      move cursor to before #;
    //      rotate; recurse
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
