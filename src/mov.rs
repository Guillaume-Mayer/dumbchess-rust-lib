use piece::PieceType;

#[derive(Debug)]
pub enum Mov {
    Quiet(usize, usize),
    TwoPush(usize),
    CastleKing,
    CastleQueen,
    Capture(usize, usize),
    _EnPassant(usize, usize),
    _Promotion(usize, usize, PieceType),
    _PromotionCapture(usize, usize, PieceType),
}
