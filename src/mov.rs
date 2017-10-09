use piece::PieceType;

#[derive(Debug)]
pub enum Mov {
    Quiet(usize, usize),
    TwoPush(usize),
    Capture(usize, usize),
    EnPassant(usize, usize),
    Promotion(usize, usize, Promotion),
    PromotionCapture(usize, usize, Promotion),
    CastleKing,
    CastleQueen,
}

#[derive(Debug)]
pub enum Promotion {
    Queen,
    Knight,
    Rook,
    Bishop,
}

impl Promotion {

    pub fn to_san(&self) -> char {
        match *self {
            Promotion::Queen => 'Q',
            Promotion::Knight => 'N',
            Promotion::Rook => 'R',
            Promotion::Bishop => 'B',
        }
    }

    pub fn to_piece_type(&self) -> PieceType {
        match *self {
            Promotion::Queen => PieceType::Queen,
            Promotion::Knight => PieceType::Knight,
            Promotion::Rook => PieceType::Rook,
            Promotion::Bishop => PieceType::Bishop,
        }
    }
    
}
