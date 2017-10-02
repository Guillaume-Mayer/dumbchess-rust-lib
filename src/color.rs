#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn swap(&self) -> Color {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}