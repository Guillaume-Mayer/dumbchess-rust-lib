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

impl PieceType {
    pub fn to_fen(&self) -> char {
        match *self {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Pawn => 'P',
        }
    }
}

#[derive(Copy, Clone)]
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

    pub fn to_fen(&self) -> char {
        match self.color {
            Color::White => self.piece.to_fen(),
            Color::Black => match self.piece {
                PieceType::King => 'k',
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                PieceType::Pawn => 'p',
            },
        }
    }

    pub fn to_char(&self) -> char {
        match self.color {
            Color::White => match self.piece {
                PieceType::King => '♔',
                PieceType::Queen => '♕',
                PieceType::Rook => '♖',
                PieceType::Bishop => '♗',
                PieceType::Knight => '♘',
                PieceType::Pawn => '♙',
            },
            Color::Black => match self.piece {
                PieceType::King => '♚',
                PieceType::Queen => '♛',
                PieceType::Rook => '♜',
                PieceType::Bishop => '♝',
                PieceType::Knight => '♞',
                PieceType::Pawn => '♟',
            },
        }
    }
}
