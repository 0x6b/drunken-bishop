use std::{fmt::Display, str::from_utf8};

use crate::{
    types::{direction::Direction, position::Position, symbols::Symbols},
    HEIGHT, WIDTH,
};

pub struct World {
    pub map: Vec<Vec<u8>>,
    moves: Vec<Direction>,
}

impl World {
    /// Create a new world from a string, expecting a sequence of hexadecimal digits.
    pub fn from(s: &str) -> Self {
        let mut world = Self {
            map: vec![vec![0u8; WIDTH]; HEIGHT],
            moves: Self::parse_commands(s),
        };
        world.simulate();
        world
    }

    fn parse_commands(s: &str) -> Vec<Direction> {
        s.as_bytes()
            .chunks(2)
            .flat_map(|pair| from_utf8(pair))
            .flat_map(|str| u8::from_str_radix(str, 16))
            .flat_map(|value| {
                [
                    (value & 0b0000_0011),
                    (value >> 2 & 0b0000_0011),
                    (value >> 4 & 0b0000_0011),
                    (value >> 6 & 0b0000_0011),
                ]
            })
            .map(Direction::from)
            .collect::<Vec<_>>()
    }

    fn simulate(&mut self) {
        let start = Position::new(8, 4);
        let mut end = start.clone();

        self.moves.iter().for_each(|dir| {
            self.map[end.y][end.x] += 1;
            end = end.apply(dir);
        });

        self.map[start.y][start.x] = 15; // Set `S`tart
        self.map[end.y][end.x] = 16; // Set `E`nd
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut drawing = String::with_capacity(WIDTH * HEIGHT);
        let border = format!("+{}+", "-".repeat(WIDTH));

        drawing.push_str(&border);
        drawing.push('\n');

        (0..HEIGHT).for_each(|i| {
            drawing.push_str(&format!(
                "|{}|\n",
                self.map[i]
                    .iter()
                    .map(Symbols::get)
                    .map(String::from)
                    .collect::<Vec<_>>()
                    .join("")
            ));
        });

        drawing.push_str(&border);

        write!(f, "{drawing}")
    }
}
