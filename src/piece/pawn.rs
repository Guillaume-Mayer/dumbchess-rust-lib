use color::Color;
use piece::_Piece;

pub struct Pawn {
    color: Color,
}

impl _Piece for Pawn {
    fn new(color: Color) -> Pawn {
        Pawn { color }
    }
    fn to_char(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }
    fn to_fen(&self) -> char {
        match self.color {
            Color::White => 'P',
            Color::Black => 'p',
        }
    }
    fn to_san(&self) -> &str {
        ""
    }
    fn color(&self) -> Color {
        self.color
    }
    fn is_pawn(&self) -> bool {
        true
    }
}