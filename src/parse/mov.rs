use piece::PieceType;
use std::str::{FromStr, Chars};

#[derive(Debug)]
pub enum Mov {
    CastleKing(Indicator),
    CastleQueen(Indicator),
    Quiet(PieceType, From, usize, Indicator),
    Capture(PieceType, From, usize, Indicator),
    Promotion(From, usize, Promotion, Indicator),
    PromotionCapture(From, usize, Promotion, Indicator),
}

#[derive(Debug)]
pub enum Indicator {
    None,
    Check,
    CheckMate,
}

#[derive(Debug)]
pub enum From {
    None,
    File(usize),
    Rank(usize),
    Full(usize)
}

#[derive(Debug)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Debug)]
pub enum Error {
    EmptyMove,
    InvalidMove,
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
            Ok(Mov::Quiet(p, From::None, parse_tile(Some(f), Some(r))?, Indicator::None))
        },
        (Some(f @ 'a'...'h'), Some(r @ '1'...'8'), Some('x')) => {
            let to = parse_tile(it.next(), it.next())?;
            let mut c = it.next();
            match parse_promotion(p, &mut c, it) {
                None => Ok(Mov::Capture(p, From::Full(parse_tile(Some(f), Some(r))?), to, parse_end(c)?)),
                Some(p) => Ok(Mov::PromotionCapture(From::Full(parse_tile(Some(f), Some(r))?), to, p, parse_end(c)?))
            }
        },
        (Some(f @ 'a'...'h'), Some(r @ '1'...'8'), Some('-')) => {
            let to = parse_tile(it.next(), it.next())?;
            let mut c = it.next();
            match parse_promotion(p, &mut c, it) {
                None => Ok(Mov::Quiet(p, From::Full(parse_tile(Some(f), Some(r))?), to, parse_end(c)?)),
                Some(p) => Ok(Mov::Promotion(From::Full(parse_tile(Some(f), Some(r))?), to, p, parse_end(c)?)),
            }
        },
        (Some(f @ 'a'...'h'), Some(r @ '1'...'8'), Some(c)) => {
            let mut c = Some(c);
            match parse_promotion(p, &mut c, it) {
                None => Ok(Mov::Quiet(p, From::None, parse_tile(Some(f), Some(r))?, parse_end(c)?)),
                Some(p) => Ok(Mov::Promotion(From::None, parse_tile(Some(f), Some(r))?, p, parse_end(c)?))
            }            
        },
        (Some(f1 @ 'a'...'h'), Some('x'), Some(f2 @ 'a'...'h')) => {
            let to = parse_tile(Some(f2), it.next())?;
            let mut c = it.next();
            match parse_promotion(p, &mut c, it) {
                None => Ok(Mov::Capture(p, From::File(parse_file(f1)?), to, parse_end(c)?)),
                Some(p) => Ok(Mov::PromotionCapture(From::File(parse_file(f1)?), to, p, parse_end(c)?)),
            }
        },
        (Some(r1 @ '1'...'8'), Some(f @ 'a'...'h'), Some(r2 @ '1'...'8')) => {
            let mut c = it.next();
            match parse_promotion(p, &mut c, it) {
                None => Ok(Mov::Quiet(p, From::Rank(parse_rank(r1)?), parse_tile(Some(f), Some(r2))?, parse_end(c)?)),
                Some(p) => Ok(Mov::Promotion(From::Rank(parse_rank(r1)?), parse_tile(Some(f), Some(r2))?, p, parse_end(c)?)),
            }           
        },
        (Some(r @ '1'...'8'), Some('x'), Some(f @ 'a'...'h')) => {
            let to = parse_tile(Some(f), it.next())?;
            let mut c = it.next();
            match parse_promotion(p, &mut c, it) {
                None => Ok(Mov::Capture(p, From::Rank(parse_rank(r)?), to, parse_end(c)?)),
                Some(prom) => Ok(Mov::PromotionCapture(From::Rank(parse_rank(r)?), to, prom, parse_end(c)?)),
            }          
        },
        (Some('x'), Some(f @ 'a'...'h'), Some(r @ '1'...'8')) if p != PieceType::Pawn => {
            Ok(Mov::Capture(p, From::None, parse_tile(Some(f), Some(r))?, parse_end(it.next())?))
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

fn parse_promotion(p: PieceType, c: &mut Option<char>, mut it: Chars) -> Option<Promotion> {
    if p == PieceType::Pawn {
        match (*c, it.next()) {
            (Some('Q'), x) => {
                *c = x;
                Some(Promotion::Queen)
            },
            (Some('N'), x) => {
                *c = x;
                Some(Promotion::Knight)
            },
            (Some('R'), x) => {
                *c = x;
                Some(Promotion::Rook)
            },
            (Some('B'), x) => {
                *c = x;
                Some(Promotion::Bishop)
            },
            (Some('='), Some('Q')) => {
                *c = it.next();
                Some(Promotion::Queen)
            },
            (Some('='), Some('N')) => {
                *c = it.next();
                Some(Promotion::Knight)
            },
            (Some('='), Some('R')) => {
                *c = it.next();
                Some(Promotion::Rook)
            },
            (Some('='), Some('B')) => {
                *c = it.next();
                Some(Promotion::Bishop)
            },
            (x, _) => {
                *c = x;
                None
            },
        }
    } else {
        None
    }
}

fn parse_file(f: char) -> Result<usize, Error> {
    "abcdefgh".find(f).ok_or(Error::InvalidMove)
}

fn parse_rank(r: char) -> Result<usize, Error> {
    "12345678".find(r).ok_or(Error::InvalidMove)
}

fn parse_tile(f: Option<char>, r: Option<char>) -> Result<usize, Error> {
    match (f, r) {
        (Some(f), Some(r)) => Ok(parse_file(f)? + parse_rank(r)? * 8),
        _ => Err(Error::InvalidMove),
    }
}