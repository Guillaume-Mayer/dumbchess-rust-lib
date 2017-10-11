use color::Color;
use piece::_Piece;

pub struct Queen {
    pub color: Color,
}

impl Queen {
    pub fn new(color: Color) -> Queen {
        Queen { color }
    }
}

impl _Piece for Queen {
    fn to_char(&self) -> char {
        match self.color {
            Color::White => '♕',
            Color::Black => '♛',
        }
    }
    fn to_fen(&self) -> char {
        match self.color {
            Color::White => 'Q',
            Color::Black => 'q',
        }
    }
    fn to_san(&self) -> &str {
        "Q"
    }
    fn color(&self) -> Color {
        self.color
    }
}