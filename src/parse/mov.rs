use piece::PieceType;
use std::str::{FromStr, Chars};

pub enum Mov {
    CastleKing(Indicator),
    CastleQueen(Indicator),
    _Quiet(PieceType, From, usize, Promotion, Indicator),
    _Capture(PieceType, From, usize, Promotion, Indicator),
}

pub enum Indicator {
    None,
    Check,
    CheckMate,
}

pub enum From {
    _None,
    _File(usize),
    _Rank(u32),
    _Full(usize, u32)
}

pub enum Promotion {
    _None,
    _Queen,
    _Rook,
    _Bishop,
    _Knight,
}

#[derive(Debug)]
pub enum Error {
    InvalidMove,
    NotImplemented,
}

impl FromStr for Mov {
    type Err = Error;
    fn from_str(s: &str) -> Result<Mov, Self::Err> {
        let mut it = s.chars();
        match it.next() {
            Some('O') => parse_castle('O', it),
            Some('0') => parse_castle('0', it),
            Some('K') => parse_move(PieceType::King, it.next(), it),
            Some('Q') => parse_move(PieceType::Queen, it.next(), it),
            Some('R') => parse_move(PieceType::Rook, it.next(), it),
            Some('B') => parse_move(PieceType::Bishop, it.next(), it),
            Some('N') => parse_move(PieceType::Knight, it.next(), it),
            Some(c) => parse_move(PieceType::Pawn, Some(c), it),
            _ => Err(Error::InvalidMove),
        }
    }
}

fn parse_castle(o: char, mut it: Chars) -> Result<Mov, Error> {
    match (it.next(), it.next()) {
        (Some('-'), Some(c)) if c == o => {
            match (it.next(), it.next()) {
                (Some('-'), Some(c)) if c == o => {
                    let i = parse_end(it.next())?;
                    Ok(Mov::CastleQueen(i))
                },
                (Some(c), None) => {
                    let i = parse_end(Some(c))?;
                    Ok(Mov::CastleKing(i))
                },
                (None, None) => Ok(Mov::CastleKing(Indicator::None)),
                _ => Err(Error::InvalidMove)
            }
        },
        _ => Err(Error::InvalidMove),
    }
}

fn parse_move(_p: PieceType, c: Option<char>, mut it: Chars) -> Result<Mov, Error> {
    match (c, it.next(), it.next()) {
        (Some(_f @ 'a'...'h'), Some(_r @ '1'...'8'), _) => Err(Error::NotImplemented),
        (Some(_f1 @ 'a'...'h'), Some('x'), Some(_f2 @ 'a'...'h')) => Err(Error::NotImplemented),
        (Some(_r1 @ '1'...'8'), Some(_f @ 'a'...'h'), Some(_r2 @ '1'...'8')) => Err(Error::NotImplemented),
        (Some(_r @ '1'...'8'), Some('x'), Some(_f @ 'a'...'h')) => Err(Error::NotImplemented),
        (Some('x'), Some(_f @ 'a'...'h'), Some(_r @ '1'...'8')) => Err(Error::NotImplemented),
        _ => Err(Error::InvalidMove),
    }
}

fn parse_end(c: Option<char>) -> Result<Indicator, Error> {
    match c {
        Some('+') => Ok(Indicator::Check),
        Some('#') => Ok(Indicator::CheckMate),
        None => Ok(Indicator::None),
        _ => Err(Error::InvalidMove),
    }
}