advent_of_code::solution!(3);
use anyhow::Result;
use regex::Regex;

pub fn part_one(input: &str) -> Result<usize> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    sum_matches(input, &re)
}

pub fn part_two(input: &str) -> Result<usize> {
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))")?;
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut mul_enabled = true;
    let mut total = 0;
    for (_, [capture]) in re.captures_iter(input).map(|c| c.extract()) {
        match capture {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            mul => {
                if mul_enabled {
                    total += sum_matches(mul, &mul_re)?
                }
            }
        }
    }
    Ok(total)}

fn sum_matches(input: &str, re: &Regex) -> Result<usize> {
    let mut total = 0;
    for (_, [lhs, rhs]) in re.captures_iter(input).map(|c| c.extract()) {
        let lhs: usize = lhs.parse()?;
        let rhs: usize = rhs.parse()?;
        total += lhs * rhs;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 161);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY))?;
        assert_eq!(result, 48);

        Ok(())
    }
}
