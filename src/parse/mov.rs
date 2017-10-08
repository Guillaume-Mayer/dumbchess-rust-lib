use piece::PieceType;
use std::str::{FromStr, Chars};

pub enum Mov {
    CastleKing(Indicator),
    CastleQueen(Indicator),
    Quiet(PieceType, From, usize, Promotion, Indicator),
    Capture(PieceType, From, usize, Promotion, Indicator),
}

pub enum Indicator {
    None,
    Check,
    CheckMate,
}

pub enum From {
    None,
    _File(usize),
    _Rank(usize),
    _Full(usize)
}

pub enum Promotion {
    None,
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Debug)]
pub enum Error {
    EmptyMove,
    InvalidMove,
    NotImplemented,
}

impl FromStr for Mov {
    type Err = Error;
    fn from_str(s: &str) -> Result<Mov, Self::Err> {
        let mut it = s.trim().chars();
        match it.next() {
            Some('O') => parse_castle('O', it),
            Some('0') => parse_castle('0', it),
            Some('o') => parse_castle('o', it),
            Some('K') => parse_move(PieceType::King, it.next(), it),
            Some('Q') => parse_move(PieceType::Queen, it.next(), it),
            Some('R') => parse_move(PieceType::Rook, it.next(), it),
            Some('B') => parse_move(PieceType::Bishop, it.next(), it),
            Some('N') => parse_move(PieceType::Knight, it.next(), it),
            Some(c) => parse_move(PieceType::Pawn, Some(c), it),
            None => Err(Error::EmptyMove),
        }
    }
}

fn parse_castle(o: char, mut it: Chars) -> Result<Mov, Error> {
    match (it.next(), it.next()) {
        (Some('-'), Some(c)) if c == o => {
            match (it.next(), it.next()) {
                (Some('-'), Some(c)) if c == o => Ok(Mov::CastleQueen(parse_end(it.next())?)),
                (Some(c), None) => Ok(Mov::CastleKing(parse_end(Some(c))?)),
                (None, None) => Ok(Mov::CastleKing(Indicator::None)),
                _ => Err(Error::InvalidMove),
            }
        },
        _ => Err(Error::InvalidMove),
    }
}

fn parse_move(p: PieceType, c: Option<char>, mut it: Chars) -> Result<Mov, Error> {
    match (c, it.next(), it.next()) {
        (Some(f @ 'a'...'h'), Some(r @ '1'...'8'), None) => {          
            Ok(Mov::Quiet(p, From::None, parse_tile(f, r), Promotion::None, Indicator::None))
        },
        (Some(_f @ 'a'...'h'), Some(_r @ '1'...'8'), Some('x')) => {
            Err(Error::NotImplemented)
        },
        (Some(_f @ 'a'...'h'), Some(_r @ '1'...'8'), Some('-')) => {
            Err(Error::NotImplemented)
        },
        (Some(f @ 'a'...'h'), Some(r @ '1'...'8'), Some(c)) => {
            let mut c = Some(c);
            let prom = if p == PieceType::Pawn {
                parse_promotion(&mut c, it)?
            } else {
                Promotion::None
            };
            Ok(Mov::Quiet(p, From::None, parse_tile(f, r), prom, parse_end(c)?))
        },
        (Some(_f1 @ 'a'...'h'), Some('x'), Some(_f2 @ 'a'...'h')) => {
            Err(Error::NotImplemented)
        },
        (Some(_r1 @ '1'...'8'), Some(_f @ 'a'...'h'), Some(_r2 @ '1'...'8')) => {
            Err(Error::NotImplemented)
        },
        (Some(_r @ '1'...'8'), Some('x'), Some(_f @ 'a'...'h')) => {
            Err(Error::NotImplemented)
        },
        (Some('x'), Some(f @ 'a'...'h'), Some(r @ '1'...'8')) if p != PieceType::Pawn => {
            let i = parse_end(it.next())?;
            Ok(Mov::Capture(p, From::None, parse_tile(f, r), Promotion::None, i))
        },
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

fn parse_promotion(c: &mut Option<char>, mut it: Chars) -> Result<Promotion, Error> {
    match (*c, it.next()) {
        (Some('Q'), x) => {
            *c = x;
            Ok(Promotion::Queen)
        },
        (Some('N'), x) => {
            *c = x;
            Ok(Promotion::Knight)
        },
        (Some('R'), x) => {
            *c = x;
            Ok(Promotion::Rook)
        },
        (Some('B'), x) => {
            *c = x;
            Ok(Promotion::Bishop)
        },
        (Some('='), Some('Q')) => {
            *c = it.next();
            Ok(Promotion::Queen)
        },
        (Some('='), Some('N')) => {
            *c = it.next();
            Ok(Promotion::Knight)
        },
        (Some('='), Some('R')) => {
            *c = it.next();
            Ok(Promotion::Rook)
        },
        (Some('='), Some('B')) => {
            *c = it.next();
            Ok(Promotion::Bishop)
        },
        (x, _) => {
            *c = x;
            Ok(Promotion::None)
        },
    }
}

fn parse_file(f: char) -> usize {
    "abcdefgh".find(f).unwrap()
}

fn parse_rank(r: char) -> usize {
    "12345678".find(r).unwrap()
}

fn parse_tile(f: char, r: char) -> usize {
    parse_file(f) + parse_rank(r) * 8
}