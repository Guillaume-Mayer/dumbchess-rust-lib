use piece::PieceType;

pub enum Mov {
    Quiet(usize, usize),
    TwoPush(usize, usize),
    CastleKing,
    CastleQueen,
    Capture(usize, usize),
    EnPassant(usize, usize),
    Promotion(usize, usize, PieceType),
    PromotionCapture(usize, usize, PieceType),
}

/*impl Mov {
    pub fn is_capture(&self) -> bool {
        match *self {
            Mov::Capture(_,_) => true,
            Mov::EnPassant(_,_) => true,
            Mov::PromotionCapture(_,_,_) => true,
            _ => false,
        }
    }
    pub fn is_promotion(&self) -> bool {
        match *self {
            Mov::Promotion(_,_,_) => true,
            Mov::PromotionCapture(_,_,_) => true,
            _ => false,
        }
    }
}*/
