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

fn digit(str: &str, position: Position) -> usize {
    let digit = position.choose_char_digit(str.chars().enumerate().filter_map(|(index, char)| char.to_digit(10).map(|d| (index, d as usize))));
    let spelled = position.choose_spelled_digit(NUMBERS.iter().filter_map(|(name, val)| position.find_spelled_digit(str, name, *val)));
    match (digit, spelled) {
        (None, None) => unreachable!("Puzzle input must contain proper data"),
        (Some((_, val)), None) | (None, Some((_, val))) => val,
        (Some((i1, v1)), Some((i2, v2))) => if position.compare(i1, i2) { v1 } else { v2 }
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
    input.lines().map(|line| digit(line, Position::First) * 10 + digit(line, Position::Last)).sum::<usize>().to_string()
}

enum Position {
    First,
    Last,
}

impl Position {
    fn choose_char_digit(&self, mut chars: impl Iterator<Item=(usize, usize)>) -> Option<(usize, usize)> {
        match self {
            Position::First => chars.next(),
            Position::Last => chars.last()
        }
    }

    fn find_spelled_digit(&self, str: &str, name: &str, val: usize) -> Option<(usize, usize)> {
        match self {
            Position::First => str.find(name).map(|i| (i, val)),
            Position::Last => str.rfind(name).map(|i| (i, val)),
        }
    }

    fn choose_spelled_digit(&self, chars: impl Iterator<Item=(usize, usize)>) -> Option<(usize, usize)> {
        match self {
            Position::First => chars.min_by(|(i1, _), (i2, _)| i1.cmp(i2)),
            Position::Last => chars.max_by(|(i1, _), (i2, _)| i1.cmp(i2))
        }
    }

    fn compare(&self, i1: usize, i2: usize) -> bool {
        match self {
            Position::First => i1 < i2,
            Position::Last => i1 > i2
        }
    }
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