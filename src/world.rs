use std::{
    error::Error,
    fmt::{Display, Formatter, Result, Write},
};

use crate::{Direction, HEIGHT, Position, Symbols, WIDTH};

pub struct World {
    pub map: Vec<Vec<u8>>,
    moves: Vec<Direction>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseHexError {
    OddLength,
    InvalidDigit { index: usize, byte: u8 },
}

impl Display for ParseHexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::OddLength => {
                f.write_str("hexadecimal input must contain an even number of digits")
            }
            Self::InvalidDigit { index, byte } => {
                write!(f, "invalid hexadecimal digit {:?} at byte {index}", *byte as char)
            }
        }
    }
}

impl Error for ParseHexError {}

impl World {
    /// Create a new world from a sequence of hexadecimal digits.
    pub fn from_hex(s: &str) -> std::result::Result<Self, ParseHexError> {
        let mut world = Self {
            map: vec![vec![0u8; WIDTH]; HEIGHT],
            moves: Self::parse_commands(s)?,
        };
        world.simulate();
        Ok(world)
    }

    fn parse_commands(s: &str) -> std::result::Result<Vec<Direction>, ParseHexError> {
        s.len()
            .is_multiple_of(2)
            .then_some(())
            .ok_or(ParseHexError::OddLength)?;

        s.as_bytes()
            .iter()
            .enumerate()
            .map(|(index, byte)| {
                hex_value(*byte).ok_or(ParseHexError::InvalidDigit { index, byte: *byte })
            })
            .collect::<std::result::Result<Vec<_>, _>>()
            .map(|digits| {
                digits
                    .chunks_exact(2)
                    .flat_map(|pair| {
                        let value = pair[0] << 4 | pair[1];
                        [
                            (value & 0b0000_0011),
                            (value >> 2 & 0b0000_0011),
                            (value >> 4 & 0b0000_0011),
                            (value >> 6 & 0b0000_0011),
                        ]
                        .map(Direction::from)
                    })
                    .collect()
            })
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

impl TryFrom<&str> for World {
    type Error = ParseHexError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Self::from_hex(value)
    }
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut drawing = String::with_capacity(WIDTH * HEIGHT);
        let border = format!("+{}+", "-".repeat(WIDTH));

        drawing.push_str(&border);
        drawing.push('\n');

        (0..HEIGHT).for_each(|i| {
            let _ = writeln!(
                drawing,
                "|{}|",
                self.map[i]
                    .iter()
                    .map(Symbols::get)
                    .map(String::from)
                    .collect::<Vec<_>>()
                    .join("")
            );
        });

        drawing.push_str(&border);

        write!(f, "{drawing}")
    }
}
