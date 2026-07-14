pub struct Symbols {}

impl Symbols {
    const TABLE: [char; 15] = [
        ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^',
    ];
    pub(crate) const MAX_VISITS: u8 = (Self::TABLE.len() - 1) as u8;

    pub(crate) fn get(i: &u8) -> char {
        Symbols::TABLE[usize::from((*i).min(Self::MAX_VISITS))]
    }
}
