use std::str::Chars;
pub type Matrix = Vec<Vec<char>>;

pub fn parse_matrix(input: &str) -> Matrix {
    input
        .lines()
        .map(|line| line.trim())
        .map(str::chars)
        .map(Chars::collect::<Vec<_>>)
        .map(|it| it.into_iter().filter(|char| char != &' ').collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect()
}

pub fn rotate(matrix: Matrix) -> Matrix {
    let (rows, columns) = matrix.dimensions();

    let mut result = vec![vec!['0'; rows]; columns];
    for row_num in 0..rows {
        for col_num in 0..columns {
            result[col_num][row_num] = matrix[rows - 1 - row_num][col_num]
        }
    }
    result
}

trait MatrixExt {
    /// Gets the dimensions of the matrix, ensuring it's square.
    fn dimensions(&self) -> (usize, usize);
}

impl MatrixExt for Matrix {
    fn dimensions(&self) -> (usize, usize) {
        let len_0 = self[0].len();
        for i in 1..self.len() {
            assert_eq!(len_0, self[i].len())
        }
        (self.len(), len_0)
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
                parse_matrix(input),
                vec![
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
            );

            Ok(())
        }
    }

    mod rotate {
        use super::*;

        #[test]
        fn it_can_rotate_a_matrix() -> Result<()> {
            assert_eq!(
                rotate(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]),
                vec![vec!['d', 'a'], vec!['e', 'b'], vec!['f', 'c']]
            );
            Ok(())
        }

        #[test]
        fn it_can_rotate_a_huge_matrix() -> Result<()> {
            let m1 = parse_matrix(
                "a b c d e f g h i j k l m
                        n o p q r s t u v w x y z",
            );

            let m2 = parse_matrix(
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
            assert_eq!(rotate(m1), m2);
            Ok(())
        }
        /// rotating 4 times returns original
        #[test]
        fn it_rotating_4_times_returns_original() -> Result<()> {
            let matrix = parse_matrix("a b c d e f g\nh i j k l m n");
            assert_eq!(matrix.clone(), rotate(rotate(rotate(rotate(matrix)))));
            Ok(())
        }
    }
}
