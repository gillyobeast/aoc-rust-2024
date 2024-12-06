use std::str::Chars;

pub fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim())
        .map(str::chars)
        .map(Chars::collect::<Vec<_>>)
        .filter(|line| !line.is_empty())
        .collect()
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
}
