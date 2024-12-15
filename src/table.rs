use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::str::Chars;

#[derive(Clone, PartialEq)]
pub struct Table {
    inner: Vec<Vec<char>>,
}

impl Table {
    pub fn empty((rows, columns): (usize, usize)) -> Table {
        Table {
            inner: vec![vec!['.'; rows]; columns],
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Iterator for Table {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.iter().cloned().next()
    }
}

impl Iterator for &Table {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.iter().cloned().next()
    }
}

impl Index<usize> for Table {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index)
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.index_mut(index)
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = "\n".to_string();
        for row in self {
            for column in row {
                out += column.to_string().as_ref()
            }
            out += "\n";
        }

        f.write_str(&out)
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

    pub fn rotated(self) -> Self {
        let (rows, columns) = self.dimensions();
        let matrix = self;

        let mut result = Self::empty((rows, columns));
        for row_num in 0..rows {
            for (col_num, column) in result.inner.iter_mut().enumerate().take(columns) {
                column[row_num] = matrix[rows - 1 - row_num][col_num]
            }
        }
        result
    }

    pub fn dimensions(&self) -> (usize, usize) {
        let len_0 = self[0].len();
        let self_len = self.len();
        for i in 1..self_len {
            assert_eq!(len_0, self[i].len())
        }
        (self_len, len_0)
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
                .rotated(),
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
            assert_eq!(m1.rotated(), m2);
            Ok(())
        }
        /// rotating 4 times returns original
        #[test]
        fn it_rotating_4_times_returns_original() -> Result<()> {
            let matrix = Table::parse(
                "a b c d e f g
                h i j k l m n",
            );
            assert_eq!(
                matrix.clone(),
                matrix.rotated().rotated().rotated().rotated()
            );
            Ok(())
        }
    }
}
