use std::fmt::{self, Display};

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
        .filter_map(|line| Game::try_from(line).ok())
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
        .filter_map(|line| Game::try_from(line).ok())
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

#[derive(Debug)]
enum ParseError {
    InvalidString(String),
    UnexpectedColor(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidString(msg) => write!(f, "Invalid input string: {msg}"),
            ParseError::UnexpectedColor(color) => write!(f, "Unexpected color `{color}`"),
        }
    }
}
impl std::error::Error for ParseError {}

#[derive(Debug, Clone, Copy)]
enum Cube {
    Red { amount: usize },
    Green { amount: usize },
    Blue { amount: usize },
}

impl TryFrom<&str> for Cube {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.trim().split(' ');
        let amount = split
            .next()
            .ok_or(ParseError::InvalidString(value.to_owned()))?
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidString(value.to_owned()))?;
        match split
            .next()
            .ok_or(ParseError::InvalidString(value.to_owned()))?
        {
            "red" => Ok(Cube::Red { amount }),
            "green" => Ok(Cube::Green { amount }),
            "blue" => Ok(Cube::Blue { amount }),
            str => Err(ParseError::UnexpectedColor(str.to_owned())),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    cubes: Vec<Cube>,
}

impl TryFrom<&str> for Game {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(":");
        let id = split
            .next()
            .ok_or(ParseError::InvalidString(value.to_owned()))?
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .expect("Digits are filtered. This cannot fail");
        let cubes = split
            .next()
            .ok_or(ParseError::InvalidString(value.to_owned()))?
            .split(|c| c == ',' || c == ';')
            .map(Cube::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Game { id, cubes })
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
