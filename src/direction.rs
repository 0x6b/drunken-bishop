use std::ops::Deref;

use crate::direction::Direction::{DownLeft, DownRight, UpLeft, UpRight};

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
}

impl Deref for Direction {
    type Target = (isize, isize);

    fn deref(&self) -> &Self::Target {
        match self {
            UpLeft => &(-1, -1),
            UpRight => &(1, -1),
            DownLeft => &(-1, 1),
            DownRight => &(1, 1),
        }
    }
}
