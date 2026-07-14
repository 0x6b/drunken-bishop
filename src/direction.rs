use crate::{DownLeft, DownRight, UpLeft, UpRight};

pub(crate) enum Direction {
    UpLeft,    // ↖
    UpRight,   // ↗
    DownLeft,  // ↙
    DownRight, // ↘
}

impl Direction {
    pub(crate) fn from(value: u8) -> Self {
        match value {
            0 => UpLeft,
            1 => UpRight,
            2 => DownLeft,
            3 => DownRight,
            _ => panic!("Invalid direction"),
        }
    }

    pub(crate) fn delta(&self) -> (isize, isize) {
        match self {
            UpLeft => (-1, -1),
            UpRight => (1, -1),
            DownLeft => (-1, 1),
            DownRight => (1, 1),
        }
    }
}
