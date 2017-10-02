use piece::PieceType;

pub enum Mov {
    Quiet(usize, usize),
    TwoPush(usize, usize),
    CastleKing(usize),
    CastleQueen(usize),
    Capture(usize, usize),
    EnPassant(usize, usize),
    Promotion(usize, usize, PieceType),
    PromotionCapture(usize, usize, PieceType),
}
