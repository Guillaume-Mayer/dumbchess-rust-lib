use color::Color;
use piece::_Piece;

pub struct Rook {
    color: Color,
}

impl _Piece for Rook {
    fn new(color: Color) -> Rook {
        Rook { color }
    }
    fn to_char(&self) -> char {
        match self.color {
            Color::White => '♖',
            Color::Black => '♜',
        }
    }
    fn to_fen(&self) -> char {
        match self.color {
            Color::White => 'R',
            Color::Black => 'r',
        }
    }
    fn to_san(&self) -> &str {
        "R"
    }
    fn color(&self) -> Color {
        self.color
    }
}