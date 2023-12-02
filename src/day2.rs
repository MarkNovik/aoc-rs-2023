pub(super) const DAY: u8 = 2;

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

pub(super) fn part1(input: &str) -> String {
    let red_cap = 12;
    let green_cap = 13;
    let blue_cap = 14;
    input
        .lines()
        .map(Game::from)
        .filter(|game| {
            game.cubes.iter().all(|&cube| match cube {
                Cube::Red { amount } => amount <= red_cap,
                Cube::Green { amount } => amount <= green_cap,
                Cube::Blue { amount } => amount <= blue_cap,
            })
        })
        .map(|g| g.id)
        .sum::<usize>()
        .to_string()
}

pub(super) fn part2(input: &str) -> String {
    input
        .lines()
        .map(Game::from)
        .map(|game| {
            let (red, green, blue) =
                game.cubes
                    .into_iter()
                    .fold((0, 0, 0), |(r, g, b), c| match c {
                        Cube::Red { amount } => (max!(r, amount), g, b),
                        Cube::Green { amount } => (r, max!(g, amount), b),
                        Cube::Blue { amount } => (r, g, max!(b, amount)),
                    });
            red * green * blue
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, Clone, Copy)]
enum Cube {
    Red { amount: usize },
    Green { amount: usize },
    Blue { amount: usize },
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let mut split = value.trim().split(' ');
        let amount = split
            .next()
            .expect("provided string must match `{amount} {color}` pattern")
            .parse::<usize>()
            .expect("`amount` must be a valid natural number");
        match split
            .next()
            .expect("provided string must match `{amount} {color}` pattern")
        {
            "red" => Cube::Red { amount },
            "green" => Cube::Green { amount },
            "blue" => Cube::Blue { amount },
            str => panic!("{str} in invalid color"),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    cubes: Vec<Cube>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut split = value.split(":");
        let id = split
            .next()
            .expect("Input must have colon")
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .expect("Digits are filtered. This cannot fail");
        let cubes = split
            .next()
            .expect("Input must have semicolon")
            .split(|c| c == ',' || c == ';')
            .map(Cube::from);
        Game { id, cubes: cubes.collect() }
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn part1test() {
        assert_eq!("8", part1(INPUT))
    }

    #[test]
    fn part2test() {
        assert_eq!("2286", part2(INPUT))
    }
}
