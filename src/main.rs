use std::fs;

macro_rules! time {
    ($b:block) => {{
        let time = std::time::Instant::now();
        let res = $b;
        (time.elapsed(), res)
    }};
}

macro_rules! run {
    ($day:ident) => {{
        print!("Day {d}: ", d = $day::DAY);
        match fs::read_to_string(format!("src/input/day{d}.txt", d = $day::DAY)) {
            Ok(input) => {
                print!("\n\tPart 1: ");
                let (time, res) = time!{{ $day::part1(&input) }};
                println!("{res}, {time:?}");
                print!("\tPart 2: ");
                let (time, res) = time!{{ $day::part2(&input) }};
                println!("{res}, {time:?}");
            }
            Err(err) => println!("Error while reading input: {err:?}")
        }
    }};
}

fn main() {
    run!(day1)
}

mod day1 {
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
        let named = NUMBERS.iter().filter_map(|(name, val)| str.find(name).map(|i| (i, *val))).max_by(|(i1, _), (i2, _)| i1.cmp(i2));
        match (digit, named) {
            (None, None) => unreachable!("Puzzle input must contain proper data"),
            (Some((_, val)), None) | (None, Some((_, val))) => val,
            (Some((i1, v1)), Some((i2, v2))) => if i1 > i2 { v1 } else { v2 }
        }
    }

    pub(super) fn part1(input: &str) -> String {
        input.lines().map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().expect("a digit in puzzle input");
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        }).sum::<u32>().to_string()
    }

    pub(super) fn part2(input: &str) -> String {
        input.lines().map(|line| first_digit(line) * 10 + last_digit(line)).sum::<usize>().to_string()
    }
}