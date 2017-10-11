use color::Color;
use piece::_Piece;

pub struct Bishop {
    color: Color,
}

impl _Piece for Bishop {
    /*fn new(color: Color) -> Bishop {
        Bishop { color }
    }*/
    fn to_char(&self) -> char {
        match self.color {
            Color::White => '♗',
            Color::Black => '♝',
        }
    }
    fn to_fen(&self) -> char {
        match self.color {
            Color::White => 'B',
            Color::Black => 'b',
        }
    }
    fn to_san(&self) -> &str {
        "B"
    }
    fn color(&self) -> Color {
        self.color
    }
}