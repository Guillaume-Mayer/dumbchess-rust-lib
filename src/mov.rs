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

trait Play {
    fn play(&self, m: Mov) -> Self;
    fn play_from_str(&self, s: String) -> Self;
    fn play_mut(&mut self, m: Mov);
    fn play_mut_from_str(&mut self, s:String);
}

use board::Board;

impl Play for Board {
    fn play(&self, m: Mov) -> Board {
        Board::new()
    }
    fn play_from_str(&self, s: String) -> Board {
        Board::new()
    }
    fn play_mut(&mut self, m: Mov) {
        unimplemented!();
    }
    fn play_mut_from_str(&mut self, s: String) {
        unimplemented!();
    }
}