use piece::PieceType;

pub enum Mov {
    Quiet(u8, u8),
    Capture(u8, u8, PieceType),
    EnPassant(u8, u8, PieceType),
    CastleKing,
    CastleQueen,
    Promotion(u8, u8, PieceType),
    PromotionCapture(u8, u8, PieceType, PieceType),
}

pub trait Play {
    fn play(&self, m: Mov) -> Self;
    fn play_mut(&mut self, m: Mov);
}

use board::Board;

impl Play for Board {
    fn play(&self, m: Mov) -> Board {
        let mut board: Board = self.clone();
        match m {
            Mov::Quiet(t1, t2) => println!("Quiet move {}, {}", t1, t2),
            Mov::Capture(t1, t2, p) => println!("Capture {}, {}, {:?}", t1, t2, p),
            Mov::EnPassant(t1, t2, p) => println!("En passant {}, {}, {:?}", t1, t2, p),
            Mov::CastleKing => println!("O-O"),
            Mov::CastleQueen => println!("O-O-O"),
            Mov::Promotion(t1, t2, p) => println!("Promotion {}, {}, {:?}", t1, t2, p),
            Mov::PromotionCapture(t1, t2, p1, p2) => println!("Promotion Capture {}, {}, {:?}, {:?}", t1, t2, p1, p2),
        }
        board
    }
    fn play_mut(&mut self, m: Mov) {
        unimplemented!();
    }
}