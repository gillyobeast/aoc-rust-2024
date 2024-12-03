advent_of_code::solution!(2);
use anyhow::Result;
pub fn part_one(input: &str) -> Result<usize> {
    let lines = parse(input);

    let count = lines.into_iter().filter(is_safe).count();
    Ok(count)
}

fn is_safe(line: &Vec<usize>) -> bool {
    let mut forward = line.clone();
    forward.sort();
    let mut backward = line.clone();
    backward.sort();
    backward.reverse();
    ((line == &forward) || (line == &backward))
        && line
            .windows(2)
            .all(|a| (1..=3).contains(&a[0].abs_diff(a[1])))
}
pub fn part_two(input: &str) -> Result<usize> {
    let lines = parse(input);

    let count = lines
        .into_iter()
        .filter(|line| subsets(line).iter().any(is_safe))
        .count();

    Ok(count)
}

fn subsets(line: &[usize]) -> Vec<Vec<usize>> {
    let mut subsets = vec![];
    for i in 0..line.len() {
        let mut clone = line.to_owned();
        clone.remove(i);
        subsets.push(clone)
    }

    subsets
}

fn parse(input: impl AsRef<str> + Sized) -> Vec<Vec<usize>> {
    let input = input.as_ref();
    let lines = input.lines();
    lines.map(get_ints).collect()
}

fn get_ints(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 2);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 4);

        Ok(())
    }
}
