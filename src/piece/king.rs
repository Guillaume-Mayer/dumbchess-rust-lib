use color::Color;
use piece::_Piece;

pub struct King {
    pub color: Color,
}

impl King {
    pub fn new(color: Color) -> King {
        King { color }
    }
}

impl _Piece for King {

    fn to_char(&self) -> char {
        match self.color {
            Color::White => '♔',
            Color::Black => '♚',
        }
    }
    fn to_fen(&self) -> char {
        match self.color {
            Color::White => 'K',
            Color::Black => 'k',
        }
    }
    fn to_san(&self) -> &str {
        "K"
    }
    fn color(&self) -> Color {
        self.color
    }
}