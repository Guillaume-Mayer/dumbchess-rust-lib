use piece::PieceType;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Mov {
    Quiet(u8, u8),
    Capture(u8, u8, PieceType),
    EnPassant(u8, u8, PieceType),
    CastleKing,
    CastleQueen,
    Promotion(u8, u8, PieceType),
    PromotionCapture(u8, u8, PieceType, PieceType),
}
