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

fn parse(c: char, mut rem: Chars) -> Result<ParsedMov, Error> {
    match c {
        'O' => parse_castle(rem),
        'K'|'Q'|'R'|'B'|'N' => parse_move(parse_piece(c), rem.next(), rem),
        _ => parse_move(PieceType::Pawn, Some(c), rem),
    }
}

fn parse_castle(rem: Chars) -> Result<ParsedMov, Error> {
    match rem.as_str() {
        "-O" => Ok(ParsedMov::CastleKing),
        "-O-O" => Ok(ParsedMov::CastleQueen),
        _ => Err(Error::InvalidMove),
    }
}

fn parse_move(p: PieceType, f: Option<char>, mut rem: Chars) -> Result<ParsedMov, Error> {
    match f {
        None => Err(Error::InvalidMove),
        Some(f) => match f {
            'a'...'h' => {
                let file = parse_file(f);
                let rank = parse_rank(rem.next().unwrap()).unwrap() as usize;
                Ok(ParsedMov::Quiet(p, rank * 8 + file))
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
