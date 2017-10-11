use color::Color;
use piece::_Piece;

pub struct Knight {
    color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Knight {
        Knight { color }
    }
}

impl _Piece for Knight {
    fn to_char(&self) -> char {
        match self.color {
            Color::White => '♘',
            Color::Black => '♞',
        }
    }
    fn to_fen(&self) -> char {
        match self.color {
            Color::White => 'N',
            Color::Black => 'n',
        }
    }
    fn to_san(&self) -> &str {
        "N"
    }
    fn color(&self) -> Color {
        self.color
    }
}