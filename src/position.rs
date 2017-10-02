use board::Board;
use color::Color;
use mov::Mov;
use super::Play;

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

    pub fn move_from_str(s: &'static str) -> Mov {
        match s {
            "e4" => Mov::TwoPush(12, 28),
            "c5" => Mov::TwoPush(50, 34),
            "Nf3" => Mov::Quiet(6, 21),
            "O-O" => Mov::CastleKing,
            "O-O-O" => Mov::CastleQueen,
            _ => unimplemented!()
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
}

impl Play for Position {
    fn play(&self, m: &Mov) -> Position {
        let half_move_clock = match *m {
            Mov::Quiet(_,_) => self.half_move_clock + 1,
            Mov::CastleKing => self.half_move_clock + 1,
            Mov::CastleQueen => self.half_move_clock + 1,
            _ => 0,
        };
        let en_passant = match *m {
            Mov::TwoPush(t1, _) => match self.color_to_play {
                Color::White => Some(t1 + 8),
                Color::Black => Some(t1 - 8),
            },
            _ => Option::None,
        };
        Position {
            board: self.board.play(m),
            color_to_play: self.color_to_play.swap(),
            half_move_clock,
            en_passant
        }
    }
}