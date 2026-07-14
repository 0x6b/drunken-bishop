use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::{Direction, HEIGHT, Position, Symbols, WIDTH};

pub struct World {
    visits: [[u8; WIDTH]; HEIGHT],
    start: Position,
    end: Position,
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
        let moves = Self::parse_commands(s)?;
        Ok(Self::simulate(&moves))
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

    fn simulate(moves: &[Direction]) -> Self {
        let start = Position::new((WIDTH / 2) as isize, (HEIGHT / 2) as isize);
        let mut world = Self { visits: [[0; WIDTH]; HEIGHT], start, end: start };

        moves.iter().for_each(|dir| {
            let visits = &mut world.visits[world.end.y][world.end.x];
            *visits = visits.saturating_add(1).min(Symbols::MAX_VISITS);
            world.end = world.end.apply(dir);
        });

        world
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

        (0..HEIGHT).for_each(|y| {
            drawing.push('|');
            (0..WIDTH).for_each(|x| {
                let position = Position { x, y };
                let symbol = if position == self.end {
                    'E'
                } else if position == self.start {
                    'S'
                } else {
                    Symbols::get(&self.visits[y][x])
                };
                drawing.push(symbol);
            });
            drawing.push('|');
            drawing.push('\n');
        });

        drawing.push_str(&border);

        write!(f, "{drawing}")
    }
}
