pub(super) const DAY: u8 = 1;

const NUMBERS: [(&str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn first_digit(str: &str) -> usize {
    let digit = str.chars().enumerate().filter_map(|(index, char)| char.to_digit(10).map(|d| (index, d as usize))).next();
    let named = NUMBERS.iter().filter_map(|(name, val)| str.find(name).map(|i| (i, *val))).min_by(|(i1, _), (i2, _)| i1.cmp(i2));
    match (digit, named) {
        (None, None) => unreachable!("Puzzle input must contain proper data"),
        (Some((_, val)), None) | (None, Some((_, val))) => val,
        (Some((i1, v1)), Some((i2, v2))) => if i1 < i2 { v1 } else { v2 }
    }
}

fn last_digit(str: &str) -> usize {
    let digit = str.chars().enumerate().filter_map(|(index, char)| char.to_digit(10).map(|d| (index, d as usize))).last();
    let named = NUMBERS.iter().filter_map(|(name, val)| str.rfind(name).map(|i| (i, *val))).max_by(|(i1, _), (i2, _)| i1.cmp(i2));
    match (digit, named) {
        (None, None) => unreachable!("Puzzle input must contain proper data"),
        (Some((_, val)), None) | (None, Some((_, val))) => val,
        (Some((i1, v1)), Some((i2, v2))) => if i1 > i2 { v1 } else { v2 }
    }
}

pub(super) fn part1(input: &str) -> String {
    input.lines().map(|line| {
        let first = line.chars().filter_map(|c| c.to_digit(10)).next().expect("Puzzle input must contain a digit");
        let last = line.chars().rev().filter_map(|c| c.to_digit(10)).next().unwrap_or(first);
        first * 10 + last
    }).sum::<u32>().to_string()
}

pub(super) fn part2(input: &str) -> String {
    input.lines().map(|line| first_digit(line) * 10 + last_digit(line)).sum::<usize>().to_string()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1test() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("142", part1(input));
    }

    #[test]
    fn part2test() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!("281", part2(input));
    }
}