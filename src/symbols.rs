pub struct Symbols {}

impl Symbols {
    const TABLE: [char; 17] = [
        ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^', 'S', 'E',
    ];
    const LEN: usize = Self::TABLE.len();

    pub(crate) fn get(i: &u8) -> char {
        Symbols::TABLE[(*i as usize) % Symbols::LEN]
    }
}
