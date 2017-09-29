use piece::PieceType;

pub enum Move {
    Quiet(u8, u8),
    Capture(u8, u8, PieceType),
    CaptureEnPassant(u8, u8),
    CastleKing,
    CastleQueen,
    Promotion(u8, u8, PieceType),
    PromotionCapture(u8, u8, PieceType, PieceType),
}
