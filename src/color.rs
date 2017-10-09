#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn swap(&self) -> Color {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn to_fen(&self) -> char {
        match *self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}
