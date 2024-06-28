use std::ops::Deref;

pub(crate) enum Direction {
    UpLeft,    // ↖
    UpRight,   // ↗
    DownLeft,  // ↙
    DownRight, // ↘
}

impl Direction {
    pub(crate) fn from(value: u8) -> Self {
        match value {
            0 => Self::UpLeft,
            1 => Self::UpRight,
            2 => Self::DownLeft,
            3 => Self::DownRight,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Deref for Direction {
    type Target = (isize, isize);

    fn deref(&self) -> &Self::Target {
        match self {
            Self::UpLeft => &(-1, -1),
            Self::UpRight => &(1, -1),
            Self::DownLeft => &(-1, 1),
            Self::DownRight => &(1, 1),
        }
    }
}
