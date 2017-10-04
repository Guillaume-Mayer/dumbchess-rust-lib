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
pub enum ParsedMov {
    CastleKing,
    CastleQueen,
    Quiet(PieceType, usize),
}

#[derive(Debug)]
pub enum Error {
    EmptyStr,
    InvalidMove,
    Unimplemented,
}

impl FromStr for ParsedMov {

    type Err = Error;

    fn from_str(s: &str) -> Result<ParsedMov, Self::Err> {
        let mut it = s.chars();
        match it.next() {
            None => Err(Error::EmptyStr),
            Some(c) => parse(c, it),
        }
    }
}

fn parse(c: char, rem: Chars) -> Result<ParsedMov, Error> {
    match c {
        'O' => parse_castle(rem),
        f @ 'a'...'h' => parse_file(f, rem),
        'K' | 'Q' | 'R' | 'B' | 'N' => parse_piece('N', rem),
        _ => Err(Error::InvalidMove),
    }
}

fn parse_castle(rem: Chars) -> Result<ParsedMov, Error> {
    match rem.as_str() {
        "-O" => Ok(ParsedMov::CastleKing),
        "-O-O" => Ok(ParsedMov::CastleQueen),
        _ => Err(Error::InvalidMove),
    }
}

fn parse_file(c: char, rem: Chars) -> Result<ParsedMov, Error> {
    println!("Unused: {:?}", rem);
    match c {
        'c' => Ok(ParsedMov::Quiet(PieceType::Pawn, 28)),
        'e' => Ok(ParsedMov::Quiet(PieceType::Pawn, 34)),
        _ => Err(Error::Unimplemented),
    }
}

fn parse_piece(p: char, rem: Chars) -> Result<ParsedMov, Error> {
    println!("Unused: {:?}", rem);
    match p {
        'N' => Ok(ParsedMov::Quiet(PieceType::Knight, 21)),
        _ => Err(Error::Unimplemented),
    }
}
