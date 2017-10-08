use parse::mov::*;
use piece::PieceType;

#[test]
fn parse_move_zz() {
    let m = "zz".parse::<Mov>();
    assert!(m.is_err());
}

#[test]
#[should_panic(expected = "InvalidMove")]
fn parse_move_panic() {
    "zz".parse::<Mov>().unwrap();
}

#[test]
fn parse_move_e4() {
    match "e4".parse() {
        Ok(Mov::_Quiet(PieceType::Pawn, From::_None, 28, Promotion::_None, Indicator::None)) => (),
        _ => assert!(false, "e4"),
    }
}

#[test]
#[allow(non_snake_case)]
fn parse_move_Nf3() {
    let m = "Nf3".parse::<Mov>();
    assert!(m.is_ok());
}

#[test]
#[allow(non_snake_case)]
fn parse_move_Nxf3() {
    let m = "Nxf3".parse::<Mov>();
    assert!(m.is_ok());
}

#[test]
fn parse_move_castle() {
    match "O-O-O".parse() {
        Ok(Mov::CastleQueen(Indicator::None)) => assert!(true),
        _ => assert!(false, "Castle queen"),
    };
    match "O-O".parse() {
        Ok(Mov::CastleKing(Indicator::None)) => assert!(true),
        _ => assert!(false, "Castle king"),
    };
    match "O-O-O+".parse() {
        Ok(Mov::CastleQueen(Indicator::Check)) => assert!(true),
        _ => assert!(false, "O-O-O+"),
    }
    match "0-0#".parse() {
        Ok(Mov::CastleKing(Indicator::CheckMate)) => assert!(true),
        _ => assert!(false, "0-0#"),
    }
    match "O-Oz".parse::<Mov>() {
        Err(Error::InvalidMove) => assert!(true),
        _ => assert!(false, "O-Oz"),
    }
    match "O-0".parse::<Mov>() {
        Err(Error::InvalidMove) => assert!(true),
        _ => assert!(false, "O-0"),
    }
    match "O-O-Oz".parse::<Mov>() {
        Err(Error::InvalidMove) => assert!(true),
        _ => assert!(false, "O-Oz"),
    }
}