#[derive(Clone, Copy, PartialEq, Eq)]
enum Draw {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Draw {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("no"),
        }
    }
}

impl Into<u32> for Draw {
    fn into(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn eval(a: Draw, b: Draw) -> u32 {
    let mut result = 0;
    let d: u32 = b.clone().into();
    result += d;
    if a == b {
        result += 3;
    } else {
        match (a, b) {
            (Draw::Paper, Draw::Scissors) => result += 6,
            (Draw::Rock, Draw::Paper) => result += 6,
            (Draw::Scissors, Draw::Rock) => result += 6,
            _ => (),
        }
    }
    result
}

fn main() {
    let input = include_str!("./../../inputs/day2.txt");
    let mut result: u32 = 0;
    let mut result2: u32 = 0;
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let draws: Vec<Draw> = line.split(" ").map(|c| c.into()).collect();
        result += eval(draws[0], draws[1]);

        let draws: Vec<&str> = line.split(" ").collect();
        let opponent: Draw = draws[0].into();
        let my_draw: Draw = match draws[1] {
            "X" => match opponent {
                Draw::Rock => Draw::Scissors,
                Draw::Paper => Draw::Rock,
                Draw::Scissors => Draw::Paper,
            },
            "Y" => opponent.clone(),
            "Z" => match opponent {
                Draw::Rock => Draw::Paper,
                Draw::Paper => Draw::Scissors,
                Draw::Scissors => Draw::Rock,
            },
            _ => panic!("no"),
        };
        result2 += eval(opponent, my_draw);
    }
    dbg!(result);
    dbg!(result2);
}
