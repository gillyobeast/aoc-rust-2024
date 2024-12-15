advent_of_code::solution!(15);

use advent_of_code::table::{Coords, Table};
use anyhow::Result;

pub fn part_one(input: &str) -> Result<usize> {
    let split: Vec<_> = input.splitn(2, "\n\n").collect();
    let mut table = Table::parse(split[0]);
    let steps: Vec<_> = split[1]
        .chars()
        .filter(|char| !char.is_whitespace())
        .map(Direction::from_char)
        .collect();
    for step in steps {
        table = move_robot_if_poss(table, step)
    }
    // sum coordinates
    Ok(sum_coords(table))
    // profit
}

fn sum_coords(table: Table) -> usize {
    let mut sum = 0;
    for row in 0..table.inner.len() {
        for column in 0..table[row].len() {
            if table[(row, column)] == 'O' {
                sum += 100 * row + column;
            }
        }
    }
    sum
}

fn move_robot_if_poss(table: Table, direction: Direction) -> Table {
    let from = table.find('@').unwrap();
    move_if_poss(table, from, direction)
}

fn move_if_poss(table: Table, from: Coords, direction: Direction) -> Table {
    let to = direction.step(from);
    match table[to] {
        '#' => table,
        '.' | 'O' | '@' => move_if_poss(table.swap(from, to), to, direction),
        _ => {
            panic!()
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn step(&self, (row, col): Coords) -> Coords {
        match self {
            Up => (row - 1, col),
            Down => (row + 1, col),
            Left => (row, col - 1),
            Right => (row, col + 1),
        }
    }
    fn from_char(char: char) -> Self {
        match char {
            '>' => Right,
            '<' => Left,
            '^' => Up,
            'v' => Down,
            _ => panic!("[{char}] is not a direction!"),
        }
    }
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
        assert_eq!(result, 10092);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 0);

        Ok(())
    }
}
