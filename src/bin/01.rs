advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let (mut first, mut second) = parse(input);

    first.sort();
    second.sort();

    let mut distance = 0;
    for i in 0..first.len() {
        let first = first[i];
        let second = second[i];
        let difference = first.abs_diff(second);

        // println!("{} - {} = {}", first, second, difference);
        distance += difference;
    }

    Some(distance)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (first, second) = parse(input);
    let mut similarity = 0;
    for i in 0..first.len() {
        let first = first[i];
        let count = second.iter().filter(|i| i == &&first).count();
        let new = count * first;
        // println!("{new} = {count} * {first}");
        similarity += new;
    }
    Some(similarity)
}

fn parse(input: impl AsRef<str> + Sized) -> (Vec<usize>, Vec<usize>) {
    let input = input.as_ref();
    let lines = input.lines();
    let first: Vec<usize> = lines.clone().map(get_int(0)).collect();
    let second: Vec<usize> = lines.clone().map(get_int(1)).collect();
    (first, second)
}

fn get_int(i: usize) -> impl Fn(&str) -> usize {
    move |line| {
        line.split_whitespace().collect::<Vec<_>>()[i]
            .parse::<usize>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
