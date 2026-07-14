use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
};

pub(crate) const WIDTH: usize = 17;
pub(crate) const HEIGHT: usize = 9;
pub(crate) const MAX_VISITS: u8 = 14;

pub struct World {
    visits: [[u8; WIDTH]; HEIGHT],
    start: Position,
    end: Position,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseHexError {
    OddLength { length: usize },
    InvalidDigit { index: usize, byte: u8 },
}

impl Display for ParseHexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OddLength { length } => write!(
                f,
                "hexadecimal input has {length} digits; add or remove one digit to make complete byte pairs"
            ),
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
            .ok_or(ParseHexError::OddLength { length: s.len() })?;

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
            *visits = visits.saturating_add(1).min(MAX_VISITS);
            world.end = world.end.apply(dir);
        });

        world
    }

    pub(crate) fn symbol_at(&self, x: usize, y: usize) -> Cell {
        let position = Position { x, y };
        match (position == self.end, position == self.start) {
            (true, _) => Cell::End,
            (false, true) => Cell::Start,
            (false, false) => Cell::Visits(self.visits[y][x]),
        }
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

pub(crate) enum Cell {
    Visits(u8),
    Start,
    End,
}

enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::UpLeft,
            1 => Self::UpRight,
            2 => Self::DownLeft,
            3 => Self::DownRight,
            _ => unreachable!("directions are encoded with two bits"),
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Self::UpLeft => (-1, -1),
            Self::UpRight => (1, -1),
            Self::DownLeft => (-1, 1),
            Self::DownRight => (1, 1),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x: x.clamp(0, (WIDTH - 1) as isize) as usize,
            y: y.clamp(0, (HEIGHT - 1) as isize) as usize,
        }
    }

    fn apply(&self, dir: &Direction) -> Position {
        let (dx, dy) = dir.delta();
        Position::new(self.x as isize + dx, self.y as isize + dy)
    }
}
