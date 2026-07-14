use std::fmt::{Display, Formatter, Result, Write};

use crate::world::{Cell, HEIGHT, MAX_VISITS, WIDTH, World};

const SYMBOLS: [char; MAX_VISITS as usize + 1] =
    [' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^'];

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "+{}+", "-".repeat(WIDTH))?;

        (0..HEIGHT).try_for_each(|y| {
            f.write_char('|')?;
            (0..WIDTH).try_for_each(|x| {
                let symbol = match self.symbol_at(x, y) {
                    Cell::Visits(visits) => SYMBOLS[usize::from(visits)],
                    Cell::Start => 'S',
                    Cell::End => 'E',
                };
                f.write_char(symbol)
            })?;
            writeln!(f, "|")
        })?;

        write!(f, "+{}+", "-".repeat(WIDTH))
    }
}
