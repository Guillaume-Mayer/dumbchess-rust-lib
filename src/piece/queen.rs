use color::Color;
use piece::_Piece;

pub struct Queen {
    color: Color,
}

impl _Piece for Queen {
    fn new(color: Color) -> Queen {
        Queen { color }
    }
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