use board::Board;
use color::Color;
use mov::{Mov, ParsedMov, Error};
use tile::Tile;
use piece::{Piece, PieceType};

pub struct Position {
    board: Board,
    color_to_play: Color,
    half_move_clock: u16,
    en_passant: Option<usize>,
}

impl Position {

    pub fn new() -> Position {
        Position {
            board: Board::new(),
            color_to_play: Color::White,
            half_move_clock: 0,
            en_passant: None,
        }
    }

    pub fn to_fen(&self) -> String {
        format!("{} {} {} {} {}", self.board.to_fen(), self.color_to_play.to_fen(), "KQkq", self.en_passant_to_fen(), self.half_move_clock)
    }

    pub fn to_str(&self) -> String {
        self.board.to_str()
    }

    pub fn move_from_str(&self, s: &str) -> Result<Mov, Error> {
        let m: ParsedMov = s.parse()?;
        match m {
            ParsedMov::CastleKing => Ok(Mov::CastleKing),
            ParsedMov::CastleQueen => Ok(Mov::CastleQueen),
            ParsedMov::Quiet(p, i2) => match p {
                PieceType::Pawn => Ok(Mov::TwoPush(i2)),
                PieceType::Knight => Ok(Mov::Quiet(6, i2)),
                _ => Err(Error::InvalidMove)
            },
            _ => Err(Error::Unimplemented)
        }
    }

    fn en_passant_to_fen(&self) -> String {
        match self.en_passant {
            None => String::from("-"),
            Some(index) => Self::index_to_str(index),
        }
    }

    fn index_to_str(index: usize) -> String {
        format!("{}{}", "abcdefgh".chars().nth(index % 8).unwrap(), index / 8 + 1)
    }

    pub fn play(&self, m: &Mov) -> Position {
        let mut board = self.board.clone();
        match *m {
            Mov::Quiet(t1, t2) | Mov::Capture(t1, t2) => board.mov(t1, t2),
            Mov::TwoPush(t2) => match self.color_to_play {
                Color::White => board.mov(t2 - 16, t2),
                Color::Black => board.mov(t2 + 16, t2),
            },
            Mov::CastleKing => {},
            Mov::CastleQueen => {},
            Mov::EnPassant(t1, t2) => {
                board.mov(t1, t2);
                match self.color_to_play {
                    Color::White => board.empty(t2 - 8),
                    Color::Black => board.empty(t2 + 8),
                };
            },
            Mov::Promotion(t1, t2, p) | Mov::PromotionCapture(t1, t2, p) => board.prom(t1, t2, Piece::new(self.color_to_play, p)),
        };
        let half_move_clock = match *m {
            Mov::Quiet(_,_) | Mov::CastleKing | Mov::CastleQueen => self.half_move_clock + 1,
            _ => 0,
        };
        let en_passant = match *m {
            Mov::TwoPush(t2) => match self.color_to_play {
                Color::White => Some(t2 - 8),
                Color::Black => Some(t2 + 8),
            },
            _ => Option::None,
        };
        Position {
            board,
            color_to_play: self.color_to_play.swap(),
            half_move_clock,
            en_passant,
        }
    }

    pub fn move_to_san(&self, m: &Mov) -> String {
        match *m {
            Mov::TwoPush(i2) => format!("{}", Self::index_to_str(i2)),
            Mov::CastleKing => String::from("O-O"),
            Mov::CastleQueen => String::from("O-O-O"),
            Mov::Quiet(i1, i2) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                Tile::Occupied(p) => format!("{}{}", p.to_fen(), Self::index_to_str(i2)),
            },
            Mov::Capture(i1, i2) | Mov::EnPassant(i1, i2) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                Tile::Occupied(p) => format!("{}x{}", p.to_fen(), Self::index_to_str(i2)),
            },
            Mov::Promotion(i1, i2, pr) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                Tile::Occupied(p) => format!("{}{}={}", p.to_fen(), Self::index_to_str(i2), pr.to_fen()),
            },
            Mov::PromotionCapture(i1, i2, pr) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                Tile::Occupied(p) => format!("{}x{}={}", p.to_fen(), Self::index_to_str(i2), pr.to_fen()),
            },
        }
    }
}
