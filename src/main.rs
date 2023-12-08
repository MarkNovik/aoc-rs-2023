mod day1;
mod day2;
mod day5;

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
        match std::fs::read_to_string(format!("src/input/day{d}.txt", d = $day::DAY)) {
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
    run!(day1);
    run!(day2);
    run!(day5);
}