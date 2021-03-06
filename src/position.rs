use board::Board;
use board::Item;
use color::Color;
use mov::Mov;
use parse::mov::Mov as ParseMov;
use parse::mov::Error as ParseError;
use parse::mov::{From};
use tile::Tile;
use piece::{Piece, PieceType};

#[derive(Debug)]
pub struct Position {
    board: Board,
    color_to_play: Color,
    half_move_clock: u16,
    en_passant: Option<usize>,
}

#[derive(Debug)]
pub enum MoveError {
    ParseMoveError(ParseError),
    _IllegalMove(&'static str),
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

    pub fn move_from_str(&self, s: &str) -> Result<Mov, MoveError> {
        let m = match s.parse() {
            Ok(m) => m,
            Err(e) => return Err(MoveError::ParseMoveError(e)),
        };
        match m {
            ParseMov::CastleKing(_) => Ok(Mov::CastleKing),
            ParseMov::CastleQueen(_) => Ok(Mov::CastleQueen),
            ParseMov::Quiet(_p, From::Full(i1), i2, _) => {
                Ok(Mov::Quiet(i1, i2))
            },
            ParseMov::Quiet(PieceType::Pawn, From::None, i2, _) => {
                Ok(Mov::TwoPush(i2))
            },
            ParseMov::Quiet(PieceType::Knight, From::None, i2, _) => {
                Ok(Mov::Quiet(6, i2))
            },
            ParseMov::Quiet(_p, _from, _i2, _) => {
                unimplemented!()
            },
            ParseMov::Capture(PieceType::Pawn, From::Full(i1), i2, _) => {
                Ok(Mov::EnPassant(i1, i2))
            },
            ParseMov::Capture(_p, From::Full(i1), i2, _) => {
                Ok(Mov::Capture(i1, i2))
            },
            ParseMov::Capture(PieceType::Pawn, _from, i2, _) => {
                Ok(Mov::EnPassant(36, i2))
            },
            ParseMov::Capture(_p, _from, _i2, _) => {
                unimplemented!()
            },
            ParseMov::Promotion(From::Full(i1), i2, p, _) => {
                Ok(Mov::Promotion(i1, i2, p))
            },
            ParseMov::Promotion(_from, _i2, _p, _) => {
                unimplemented!()
            },
            ParseMov::PromotionCapture(From::Full(i1), i2, p, _) => {
                Ok(Mov::PromotionCapture(i1, i2, p))
            },
            ParseMov::PromotionCapture(_from, _i2, _p, _) => {
                unimplemented!()
            },
        }
    }

    fn en_passant_to_fen(&self) -> String {
        match self.en_passant {
            None => String::from("-"),
            Some(index) => Self::index_to_str(index),
        }
    }

    fn index_to_str(index: usize) -> String {
        format!("{}{}", Self::index_to_file(index), index / 8 + 1)
    }

    fn index_to_file(index: usize) -> char {
        "abcdefgh".chars().nth(index % 8).unwrap()
    }

    pub fn play(&self, m: &Mov) -> Position {
        let mut board = self.board.clone();
        match *m {
            Mov::Quiet(t1, t2) | Mov::Capture(t1, t2) => board.mov(t1, t2),
            Mov::TwoPush(t2) => match self.color_to_play {
                Color::White => board.mov(t2 - 16, t2),
                Color::Black => board.mov(t2 + 16, t2),
            },
            Mov::CastleKing => match self.color_to_play {
                Color::White => {
                    board.mov(4, 6);
                    board.mov(7, 5);
                },
                Color::Black => {
                    board.mov(60, 62);
                    board.mov(63, 61);
                },
            },
            Mov::CastleQueen => match self.color_to_play {
                Color::White => {
                    board.mov(4, 2);
                    board.mov(0, 3);
                },
                Color::Black => {
                    board.mov(60, 58);
                    board.mov(56, 59);
                },
            },
            Mov::EnPassant(t1, t2) => {
                board.mov(t1, t2);
                match self.color_to_play {
                    Color::White => board.empty(t2 - 8),
                    Color::Black => board.empty(t2 + 8),
                };
            },
            Mov::Promotion(t1, t2, ref p) | Mov::PromotionCapture(t1, t2, ref p) => board.prom(t1, t2, Piece::new(self.color_to_play, p.to_piece_type())),
        };
        let half_move_clock = match *m {
            Mov::CastleKing | Mov::CastleQueen => self.half_move_clock + 1,
            Mov::Quiet(_, i2) => match board.tile_at(i2) {
                Tile::Occupied(p) if p.is_pawn() => 0,
                _ => self.half_move_clock + 1,
            },
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
                Tile::Occupied(p) => format!("{}{}", p.to_san(), Self::index_to_str(i2)),
            },
            Mov::Capture(i1, i2) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                Tile::Occupied(p) => format!("{}x{}", p.to_san(), Self::index_to_str(i2)),
            },
            Mov::EnPassant(i1, i2) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                _ => format!("{}x{}", Self::index_to_file(i1), Self::index_to_str(i2)),
            },
            Mov::Promotion(i1, i2, ref pr) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                Tile::Occupied(p) => format!("{}{}={}", p.to_san(), Self::index_to_str(i2), pr.to_san()),
            },
            Mov::PromotionCapture(i1, i2, ref pr) => match self.board.tile_at(i1) {
                Tile::Empty => panic!("Empty tile"),
                Tile::Occupied(p) => format!("{}x{}={}", p.to_san(), Self::index_to_str(i2), pr.to_san()),
            },
        }
    }

    pub fn moves(&self) -> Vec<Mov> {
        self.board.pieces(self.color_to_play)
            .flat_map(|p| self.moves_piece(p))
            .collect()
    }

    fn moves_piece(&self, p: Item) -> Vec<Mov> {
        match p.piece {
            PieceType::Pawn => self.moves_pawn(p.index),
            PieceType::Knight => self.moves_knight(p.index),
            _ => vec![],
        }
    }

    fn moves_pawn(&self, i: usize) -> Vec<Mov> {
        vec![Mov::Quiet(i, i + 8), Mov::TwoPush(i + 16)]
    }

    fn moves_knight(&self, i: usize) -> Vec<Mov> {
        vec![Mov::Quiet(i, i + 15), Mov::Quiet(i, i + 17)]
    }
}
