use parse::mov::*;
use piece::PieceType;

#[test]
fn parse_move_empty() {
    match "".parse::<Mov>() {
        Err(Error::EmptyMove) => (),
        _ => assert!(false, "empty string"),
    };
    match " ".parse::<Mov>() {
        Err(Error::EmptyMove) => (),
        _ => assert!(false, "empty string"),
    };
}

#[test]
fn parse_move_zz() {
    match "zz".parse::<Mov>() {
        Err(Error::InvalidMove) => (),
        _ => assert!(false, "zz"),
    }
}

#[test]
fn parse_move_e4() {
    match "e4".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::None, 28, Promotion::None, Indicator::None)) => (),
        _ => assert!(false, "e4"),
    };
    match "e4+".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::None, 28, Promotion::None, Indicator::Check)) => (),
        _ => assert!(false, "e4+"),
    };
    match "e4#".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::None, 28, Promotion::None, Indicator::CheckMate)) => (),
        _ => assert!(false, "e4#"),
    };
    match "e4z".parse::<Mov>() {
        Err(Error::InvalidMove) => (),
        _ => assert!(false, "e4z"),
    };
}

#[test]
fn parse_e2_e4() {
    match "e2-e4".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::Full(12), 28, Promotion::None, Indicator::None)) => {},
        _ => assert!(false, "e2-e4"),
    };
    match "e2-e4+".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::Full(12), 28, Promotion::None, Indicator::Check)) => {},
        _ => assert!(false, "e2-e4+"),
    };
    match "e2-e4#".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::Full(12), 28, Promotion::None, Indicator::CheckMate)) => {},
        _ => assert!(false, "e2-e4#"),
    };
}

#[test]
fn parse_e7_e8() {
    match "e7-e8R".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::Full(52), 60, Promotion::Rook, Indicator::None)) => {},
        _ => assert!(false, "e2-e4Q+"),
    };
    match "e7-e8Q+".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::Full(52), 60, Promotion::Queen, Indicator::Check)) => {},
        _ => assert!(false, "e2-e4Q+"),
    };
    match "e7-e8=N#".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::Full(52), 60, Promotion::Knight, Indicator::CheckMate)) => {},
        _ => assert!(false, "e2-e4=N#"),
    };
    match "e7-e8=B".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::Full(52), 60, Promotion::Bishop, Indicator::None)) => {},
        _ => assert!(false, "e2-e4=N#"),
    };
}

#[test]
#[allow(non_snake_case)]
fn parse_Qd1xe2() {
    match "Qd1xe2".parse() {
        Ok(Mov::Capture(PieceType::Queen, From::Full(3), 12, Promotion::None, Indicator::None)) => {},
        _ => assert!(false, "Qd1xe2"),
    };
    match "Qd1xe2+".parse() {
        Ok(Mov::Capture(PieceType::Queen, From::Full(3), 12, Promotion::None, Indicator::Check)) => {},
        _ => assert!(false, "Qd1xe2+"),
    };
    match "Qd1xe2#".parse() {
        Ok(Mov::Capture(PieceType::Queen, From::Full(3), 12, Promotion::None, Indicator::CheckMate)) => {},
        _ => assert!(false, "Qd1xe2#"),
    };
}

#[test]
fn parse_exd5() {
    match "exd5".parse() {
        Ok(Mov::Capture(PieceType::Pawn, From::File(4), 35, Promotion::None, Indicator::None)) => {},
        _ => assert!(false, "exd5"),
    };
}

#[test]
fn parse_promotion() {
    match "e8=Q".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::None, 60, Promotion::Queen, Indicator::None)) => (),
        _ => assert!(false, "e8=Q"),
    };
    match "e8=R+".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::None, 60, Promotion::Rook, Indicator::Check)) => (),
        _ => assert!(false, "e8=Q+"),
    };
    match "e8N#".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::None, 60, Promotion::Knight, Indicator::CheckMate)) => (),
        _ => assert!(false, "e8N#"),
    };
    match "e8B".parse() {
        Ok(Mov::Quiet(PieceType::Pawn, From::None, 60, Promotion::Bishop, Indicator::None)) => (),
        _ => assert!(false, "e8N#"),
    };
    match "e8K".parse::<Mov>() {
        Err(Error::InvalidMove) => (),
        _ => assert!(false, "e8K"),
    };
    match "e8=P".parse::<Mov>() {
        Err(Error::InvalidMove) => (),
        _ => assert!(false, "e8K"),
    };
}

#[test]
#[allow(non_snake_case)]
fn parse_move_Nf3() {
    match "Nf3".parse() {
        Ok(Mov::Quiet(PieceType::Knight, From::None, 21, Promotion::None, Indicator::None)) => (),
        _ => assert!(false, "Nf3"),
    }
}

#[test]
#[allow(non_snake_case)]
fn parse_move_Nxf3() {
    match "Nxf3".parse() {
        Ok(Mov::Capture(PieceType::Knight, From::None, 21, Promotion::None, Indicator::None)) => (),
        _ => assert!(false, "Nxf3"),
    }
}

#[test]
fn parse_move_castle() {
    match "O-O-O".parse() {
        Ok(Mov::CastleQueen(Indicator::None)) => (),
        _ => assert!(false, "Castle queen"),
    };
    match "O-O".parse() {
        Ok(Mov::CastleKing(Indicator::None)) => (),
        _ => assert!(false, "Castle king"),
    };
    match "O-O-O+".parse() {
        Ok(Mov::CastleQueen(Indicator::Check)) => (),
        _ => assert!(false, "O-O-O+"),
    }
    match "0-0#".parse() {
        Ok(Mov::CastleKing(Indicator::CheckMate)) => (),
        _ => assert!(false, "0-0#"),
    }
    match "O-Oz".parse::<Mov>() {
        Err(Error::InvalidMove) => (),
        _ => assert!(false, "O-Oz"),
    }
    match "O-0".parse::<Mov>() {
        Err(Error::InvalidMove) => (),
        _ => assert!(false, "O-0"),
    }
    match "O-O-Oz".parse::<Mov>() {
        Err(Error::InvalidMove) => (),
        _ => assert!(false, "O-Oz"),
    }
}