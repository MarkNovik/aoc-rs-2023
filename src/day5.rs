use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSlice;

pub(super) const DAY: u8 = 5;

pub(super) fn part1(input: &str) -> String {
    let seeds = seeds(input);
    let almanac = Almanac::parse(input);
    seeds.into_iter().map(|seed| almanac.locate_seed(seed)).min().expect("min seed").to_string()
}

pub(super) fn part2(input: &str) -> String {
    let seeds = seeds(input);
    let almanac = Almanac::parse(input);
    seeds.par_chunks(2).map(|sl| {
        let start = sl[0];
        let length = sl[1];
        (start..(start + length)).par_bridge().map(|seed| almanac.locate_seed(seed)).min().expect("min seed of range")
    }).min().expect("min seed of min seeds of range").to_string()
}

const ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn seeds(input: &str) -> Vec<i128> {
    input
        .lines().next().expect("input has one line")
        .strip_prefix("seeds: ").expect("input has `seeds` on first line")
        .trim().split_ascii_whitespace()
        .map(|s| s.parse::<i128>().expect("seed is num"))
        .collect()
}

#[derive(Copy, Clone, Debug)]
struct Range {
    src: i128,
    dest: i128,
    length: i128,
}

impl Range {
    fn new(src: i128, dest: i128, length: i128) -> Self {
        Self { src, dest, length }
    }

    fn map(&self, item: i128) -> Option<i128> {
        let &Range { src, dest, length } = self;
        if item >= src && item < (src + length) {
            Some(dest - (src - item))
        } else {
            None
        }
    }
}

struct AlmanacSection(Vec<Range>);

impl AlmanacSection {
    fn map(&self, item: i128) -> i128 {
        self.0.iter().filter_map(|range| range.map(item)).next().unwrap_or(item)
    }
}

struct Almanac([AlmanacSection; 7]);

impl Almanac {
    fn parse(input: &str) -> Almanac {
        Almanac(ORDER.map(|section| {
            let mut start = input.lines().skip_while(|line| !line.starts_with(&format!("{section} map:"))).take_while(|line| !line.is_empty());
            let _header = start.next();
            AlmanacSection(
                start.map(|line| {
                    let mut nums = line.split_ascii_whitespace().map(|s| s.parse::<i128>().expect("num"));
                    let dest = nums.next().expect("dest");
                    let src = nums.next().expect("src");
                    let length = nums.next().expect("length");
                    Range::new(src, dest, length)
                }).collect()
            )
        }))
    }

    fn locate_seed(&self, seed: i128) -> i128 {
        self.0.iter().fold(seed, |acc, section| section.map(acc))
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn  part1test() {
        assert_eq!("35", super::part1(INPUT))
    }
}