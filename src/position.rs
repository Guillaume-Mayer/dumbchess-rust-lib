use board::Board;
use color::Color;
use mov::Mov;
use tile::Tile;

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

    pub fn move_from_str(&self, s: &'static str) -> Mov {
        match s {
            "e4" => Mov::TwoPush(12, 28),
            "c5" => Mov::TwoPush(50, 34),
            "Nf3" => Mov::Quiet(6, 21),
            "O-O" => Mov::CastleKing(match self.color_to_play {
                Color::White => 4,
                Color::Black => 60,
            }),
            "O-O-O" => Mov::CastleQueen(match self.color_to_play {
                Color::White => 4,
                Color::Black => 60,
            }),
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

    pub fn play(&self, m: &Mov) -> Position {
        let half_move_clock = match *m {
            Mov::Quiet(_,_) | Mov::CastleKing(_) | Mov::CastleQueen(_) => self.half_move_clock + 1,
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

    pub fn move_to_san(&self, m: &Mov) -> String {
        match *m {
            Mov::TwoPush(t1, t2) => format!("{}", Self::index_to_str(t2)),
            Mov::CastleKing(_) => String::from("O-O"),
            Mov::CastleQueen(_) => String::from("O-O-O"),
            Mov::Quiet(t1, t2) => {
                match self.board.tile_at(t1) {
                    Tile::Empty => panic!("Empty tile"),
                    Tile::Occupied(p) => format!("{}{}", p.to_fen(), Self::index_to_str(t2)),
                }
            },
            _ => unimplemented!(),
        }
    }
}
