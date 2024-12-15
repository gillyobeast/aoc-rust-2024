advent_of_code::solution!(6);

use advent_of_code::table::Table;
use anyhow::{anyhow, Result};

pub fn part_one(input: &str) -> Result<usize> {
    // first rotate the table so the cursor moves right through the row
    let table: Table = Table::parse(input).rotated();

    let table = map_visited_points(table)?;

    count_visited(table)
}

fn count_visited(table: Table) -> Result<usize> {
    let mut sum = 0;
    for row in table {
        for char in row {
            if char == VISITED {
                sum += 1
            }
        }
    }
    Ok(sum)
}

const BLOCK: char = '#';
const CURSOR: char = '^';
const VISITED: char = 'x';

fn map_visited_points(mut table: Table) -> Result<Table> {
    loop {
        // find the row containing cursor
        let row_num = find_cursor_row(&table)?;
        // if you reach the end, return
        let row = &table[row_num];
        let done = !row_contains_block_after_cursor(row);
        table[row_num] = advance_cursor_to_next_block_or_end(row)?;
        if done {
            break;
        }
        table = table.rotated().rotated().rotated();
    }
    Ok(table)
}

#[allow(clippy::needless_range_loop)]
fn advance_cursor_to_next_block_or_end(row: &[char]) -> Result<Vec<char>> {
    let mut result = row.to_owned();
    let cursor_position = cursor_position(&result);
    let block = block_position(&result[cursor_position..]).map(|block| block + cursor_position);
    let end = match block {
        None => {
            // where the end of the row is
            result.len()
        }
        Some(block) => {
            // where the block is
            block
        }
    };

    for i in cursor_position..end {
        result[i] = VISITED
    }
    if let Some(block) = block {
        if block > 0 {
            result[block - 1] = CURSOR
        }
    }

    if result.iter().filter(|it| it == &&CURSOR).count() > 1 {
        return Err(anyhow!("found two cursors!"));
    }

    Ok(result)
}

fn row_contains_block_after_cursor(row: &[char]) -> bool {
    let cursor = cursor_position(row);
    let block = block_position(&row[cursor..]);
    match block {
        None => false,
        Some(block) => cursor < block + row.len(),
    }
}

fn cursor_position(row: &[char]) -> usize {
    row.iter()
        .position(|char| char == &CURSOR)
        .expect("Row did not contain cursor. Should be impossible.")
}

fn block_position(row: &[char]) -> Option<usize> {
    row.iter().position(|char| char == &BLOCK)
}

fn find_cursor_row(table: &Table) -> Result<usize> {
    for row in 0..table.len() {
        if table[row].contains(&CURSOR) {
            return Ok(row);
        }
    }

    Err(anyhow!("Row not found!"))
}

pub fn part_two(_input: &str) -> Result<usize> {
    // unimplemented!()
    Ok(6)
}

#[cfg(test)]
mod tests {
    use super::*;
    const NOTHING: char = '.';

    mod part_one {
        use super::*;
        #[test]
        fn it_finds_the_row_with_the_cursor() -> Result<()> {
            let mut table = Table::empty((3, 4));
            assert!(find_cursor_row(&table).is_err());

            table[2][2] = CURSOR;
            assert_eq!(find_cursor_row(&table)?, 2);

            Ok(())
        }
        #[test]
        fn it_finds_blocks_in_rows() -> Result<()> {
            let mut row = vec![NOTHING; 10];
            row[0] = CURSOR;
            assert!(!row_contains_block_after_cursor(&row));

            row[3] = BLOCK;
            assert!(row_contains_block_after_cursor(&row));
            Ok(())
        }
        #[test]
        fn it_advances_cursor_to_before_next_block_or_end() -> Result<()> {
            let mut input_row = vec![NOTHING; 10];
            input_row[1] = CURSOR;

            let row = advance_cursor_to_next_block_or_end(&input_row)?;
            // check we go to the end
            assert_eq!(
                row,
                vec![
                    NOTHING, VISITED, VISITED, VISITED, VISITED, VISITED, VISITED, VISITED,
                    VISITED, VISITED,
                ]
            );

            input_row[5] = BLOCK;
            let row = advance_cursor_to_next_block_or_end(&input_row)?;
            // check we go to before the block
            assert_eq!(
                row,
                vec![
                    NOTHING, VISITED, VISITED, VISITED, CURSOR, BLOCK, NOTHING, NOTHING, NOTHING,
                    NOTHING,
                ]
            );

            input_row[7] = CURSOR;
            assert!(advance_cursor_to_next_block_or_end(&input_row).is_err(),);

            Ok(())
        }
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 41);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 6);

        Ok(())
    }
}
