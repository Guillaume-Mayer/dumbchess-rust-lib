use piece::PieceType;
use std::str::{FromStr, Chars};

#[derive(Debug)]
pub enum Mov {
    Quiet(usize, usize),
    TwoPush(usize),
    CastleKing,
    CastleQueen,
    Capture(usize, usize),
    EnPassant(usize, usize),
    Promotion(usize, usize, PieceType),
    PromotionCapture(usize, usize, PieceType),
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

fn parse(c: char, rem: Chars) -> Result<Mov, Error> {
    match c {
        'O' => parse_castle(rem),
        f @ 'a'...'h' => parse_file(f, rem),
        'K' | 'Q' | 'R' | 'B' | 'N' => parse_piece('N', rem),
        _ => Err(Error::InvalidMove),
    }
}

fn parse_castle(rem: Chars) -> Result<Mov, Error> {
    match rem.as_str() {
        "-O" => Ok(Mov::CastleKing),
        "-O-O" => Ok(Mov::CastleQueen),
        _ => Err(Error::InvalidMove),
    }
}

fn parse_file(c: char, rem: Chars) -> Result<Mov, Error> {
    match c {
        'c' => Ok(Mov::TwoPush(28)),
        'e' => Ok(Mov::TwoPush(34)),
        _ => Err(Error::Unimplemented),
    }
}

fn parse_piece(p: char, rem: Chars) -> Result<Mov, Error> {
    match p {
        'N' => Ok(Mov::Quiet(6, 21)),
        _ => Err(Error::Unimplemented),
    }
}
