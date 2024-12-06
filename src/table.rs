use std::fmt::{Debug, Formatter};
use std::str::Chars;

#[derive(Clone, PartialEq)]
pub struct Table {
    inner: Vec<Vec<char>>,
}

impl Debug for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_string();
        for row in &self.inner {
            for column in row {
                out += column.to_string().as_ref()
            }
            out += "\n";
        }

        f.write_str(&*out)
    }
}

impl Table {
    pub fn parse(input: &str) -> Self {
        Self {
            inner: input
                .lines()
                .map(|line| line.trim())
                .map(str::chars)
                .map(Chars::collect::<Vec<_>>)
                .map(|it| it.into_iter().filter(|char| char != &' ').collect())
                .filter(|line: &Vec<char>| !line.is_empty())
                .collect(),
        }
    }

    pub fn rotate(self) -> Self {
        let (rows, columns) = self.dimensions();
        let matrix = self.inner;

        let mut result = vec![vec!['0'; rows]; columns];
        for row_num in 0..rows {
            for col_num in 0..columns {
                result[col_num][row_num] = matrix[rows - 1 - row_num][col_num]
            }
        }
        Self { inner: result }
    }

    fn dimensions(&self) -> (usize, usize) {
        let len_0 = self.inner[0].len();
        for i in 1..self.inner.len() {
            assert_eq!(len_0, self.inner[i].len())
        }
        (self.inner.len(), len_0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    mod parse {
        use super::*;

        #[test]
        fn it_parses_lines_to_vec_of_vec_of_char() -> Result<()> {
            let input = "
                ....#.....
                .........#
                ..........
                ..#.......
                .......#..
                ..........
                .#..^.....
                ........#.
                #.........
                ......#...";
            assert_eq!(
                Table::parse(input),
                Table {
                    inner: vec![
                        vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
                        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                        vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
                        vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                        vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
                        vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
                        vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                        vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']
                    ]
                }
            );

            Ok(())
        }
    }

    mod rotate {
        use super::*;

        #[test]
        fn it_can_rotate_a_matrix() -> Result<()> {
            assert_eq!(
                Table {
                    inner: vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]
                }
                .rotate(),
                Table {
                    inner: vec![vec!['d', 'a'], vec!['e', 'b'], vec!['f', 'c']]
                }
            );
            Ok(())
        }

        #[test]
        fn it_can_rotate_a_huge_matrix() -> Result<()> {
            let m1 = Table::parse(
                "a b c d e f g h i j k l m
                        n o p q r s t u v w x y z",
            );

            let m2 = Table::parse(
                "n a
                        o b
                        p c
                        q d
                        r e
                        s f
                        t g
                        u h
                        v i
                        w j
                        x k
                        y l
                        z m",
            );
            assert_eq!(m1.rotate(), m2);
            Ok(())
        }
        /// rotating 4 times returns original
        #[test]
        fn it_rotating_4_times_returns_original() -> Result<()> {
            let matrix = Table::parse(
                "a b c d e f g
                h i j k l m n",
            );
            assert_eq!(matrix.clone(), matrix.rotate().rotate().rotate().rotate());
            Ok(())
        }
    }
}
