use color::Color;

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    color: Color,
    piece: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece: PieceType) -> Piece {
        Piece {
            color,
            piece,
        }
    } 
    pub fn to_char(&self) -> char {
        use self::PieceType::*;
        match self.color {
            Color::White => match self.piece {
                King => 'K',
                Queen => 'Q',
                Rook => 'R',
                Bishop => 'B',
                Knight => 'N',
                Pawn => 'P',
            },
            Color::Black => match self.piece {
                King => 'k',
                Queen => 'q',
                Rook => 'r',
                Bishop => 'b',
                Knight => 'n',
                Pawn => 'p',
            }, 
        }
    }
}
