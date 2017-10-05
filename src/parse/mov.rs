use piece::PieceType;
use std::str::{FromStr, Chars};

#[derive(Debug)]
pub enum Mov {
    CastleKing,
    CastleQueen,
    Quiet(PieceType, usize),
    QuietWithFile(PieceType, usize, usize),
    QuietWithRank(PieceType, usize, usize),
    Capture(PieceType, usize),
    CaptureWithFile(PieceType, usize, usize),
    CaptureWithRank(PieceType, usize, usize),
}

#[derive(Debug)]
pub enum Error {
    EmptyStr,
    InvalidMove,
    Unimplemented,
}

impl FromStr for Mov {

    type Err = Error;

    fn from_str(s: &str) -> Result<Mov, Self::Err> {
        let mut it = s.chars();
        match it.next() {
            None => Err(Error::EmptyStr),
            Some(c) => parse(c, it),
        }
    }
}

fn parse(c: char, mut rem: Chars) -> Result<Mov, Error> {
    match c {
        'O' => parse_castle(rem),
        'K'|'Q'|'R'|'B'|'N' => parse_move(parse_piece(c), rem.next(), rem),
        _ => parse_move(PieceType::Pawn, Some(c), rem),
    }
}

fn parse_castle(rem: Chars) -> Result<Mov, Error> {
    match rem.as_str() {
        "-O" => Ok(Mov::CastleKing),
        "-O-O" => Ok(Mov::CastleQueen),
        _ => Err(Error::InvalidMove),
    }
}

fn parse_move(p: PieceType, f: Option<char>, mut rem: Chars) -> Result<Mov, Error> {
    match f {
        None => Err(Error::InvalidMove),
        Some(f) => match f {
            'a'...'h' => {
                let file = parse_file(f);
                let rank = parse_rank(rem.next().unwrap()).unwrap() as usize;
                Ok(Mov::Quiet(p, rank * 8 + file))
            },
            _ => Err(Error::InvalidMove),
        },
    }
}

fn parse_file(c: char) -> usize {
    "abcdefgh".find(c).unwrap()
}

fn parse_rank(c: char) -> Option<u32> {
    match c.to_digit(10) {
        None => None,
        Some(r) => Some(r - 1),
    }
}

fn parse_piece(p: char) -> PieceType {
    match p {
        'K' => PieceType::King,
        'Q' => PieceType::Queen,
        'R' => PieceType::Rook,
        'B' => PieceType::Bishop,
        'N' => PieceType::Knight,
        _ => panic!("Unknow piece char"),
    }
}
