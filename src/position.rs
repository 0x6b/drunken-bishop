use crate::{Direction, HEIGHT, WIDTH};

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Position {
    pub(crate) fn new(x: isize, y: isize) -> Self {
        Self {
            x: x.clamp(0, (WIDTH - 1) as isize) as usize,
            y: y.clamp(0, (HEIGHT - 1) as isize) as usize,
        }
    }

    pub(crate) fn apply(&self, dir: &Direction) -> Position {
        let (dx, dy) = dir.delta();
        Position::new(self.x as isize + dx, self.y as isize + dy)
    }
}
