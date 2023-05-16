use std::str::FromStr;
use aoc_2022::read_lines;

use anyhow::{Error, Result};

pub enum Shape {
    Rock,
    Papper,
    Scissors,
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Papper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("uknown shape")
        }
    }
}

impl Shape {
    const ROCK_VAL: u32 = 1;
    const PAPPER_VAL: u32 = 2;
    const SCISSORS_VAL: u32 = 3;

    pub fn value(self) -> u32 {
        match self {
            Self::Rock => Self::ROCK_VAL,
            Self::Papper => Self::PAPPER_VAL,
            Self::Scissors => Self::SCISSORS_VAL,
        }
    }
}

pub enum RoundOutcome {
    Win(Shape),
    Draw(Shape),
    Lose(Shape),
}

// impl FromStr for RoundOutcome {
//     type Err = Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let shape: Shape = s.trim()
//             .chars()
//             .last()
//             .unwrap()
//             .into();
//
//         match s {
//             "C X" | "A Y" | "B Z" => Ok(Self::Win(shape)),
//             "A X" | "X A" | "B Y" | "Y B" | "C Z" | "Z C" => Ok(Self::Draw(shape)),
//             _ => Ok(Self::Lose(shape))
//         }
//     }
// }

impl FromStr for RoundOutcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();
        let op_shape: Shape = chars
            .next()
            .unwrap()
            .into();
        let res = chars.last().unwrap();

        match res {
            'Y' => Ok(Self::Draw(op_shape)),
            'X' => {
                let shape = match op_shape {
                    Shape::Rock => Shape::Scissors,
                    Shape::Papper => Shape::Rock,
                    Shape::Scissors => Shape::Papper,
                };

                Ok(Self::Lose(shape))
            }
            'Z' => {
                let shape = match op_shape {
                    Shape::Rock => Shape::Papper,
                    Shape::Papper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                };

                Ok(Self::Win(shape))
            }
            _ => Err(Error::msg("uknown result from round"))
        }
    }
}

impl RoundOutcome {
    const WIN_VAL: u32 = 6;
    const DRAW_VAL: u32 = 3;
    const LOSE_VAL: u32 = 0;

    pub fn value(self) -> u32 {
        match self {
            Self::Win(v) => v.value() + Self::WIN_VAL,
            Self::Draw(v) => v.value() + Self::DRAW_VAL,
            Self::Lose(v) => v.value() + Self::LOSE_VAL,
        }
    }
}

fn cal_game_outcome() -> Result<u32> {
    Ok(read_lines::<RoundOutcome>("day2_input.txt")?
        .into_iter()
        .map(|n| n.value())
        .sum())
}

fn main() -> Result<()> {
    println!("Game outcome: {}", cal_game_outcome()?);
    Ok(())
}
