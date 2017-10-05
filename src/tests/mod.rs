use super::game::Game;
use super::parse::mov::ParsedMov;

#[test]
fn init_fen() {
    let g = Game::new();
    assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", g.to_fen());
}

#[test]
fn play_e4_fen() {
    let mut g = Game::new();
    g.play("e4");
    assert_eq!("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1", g.to_fen());
}

#[test]
fn play_e4_c5_fen() {
    let mut g = Game::new();
    g.play("e4");
    g.play("c5");
    assert_eq!("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2", g.to_fen());
}

#[test]
fn play_e4_c5_nf3_fen() {
    let mut g = Game::new();
    g.play("e4");
    g.play("c5");
    g.play("Nf3");
    assert_eq!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2", g.to_fen());
}

#[test]
fn play_e4_c5_nf3_pgn() {
    let mut g = Game::new();
    g.play("e4");
    g.play("c5");
    g.play("Nf3");
    assert_eq!("[Event \"?\"]\n\
                [Site \"?\"]\n\
                [Date \"????.??.??\"]\n\
                [Round \"?\"]\n\
                [White \"You\"]\n\
                [Black \"Me\"]\n\
                [Result \"*\"]\n\n\
                1.e4 c5 2.Nf3 ", g.to_pgn());
}

#[test]
fn parse_move_zz() {
    let m = "zz".parse::<ParsedMov>();
    assert!(m.is_err());
}

#[test]
#[should_panic(expected = "InvalidMove")]
fn parse_move_panic() {
    "zz".parse::<ParsedMov>().unwrap();
}

#[test]
fn parse_move_e4() {
    let m = "e4".parse::<ParsedMov>();
    assert!(m.is_ok());
}

#[test]
#[allow(non_snake_case)]
fn parse_move_Nf3() {
    let m = "Nf3".parse::<ParsedMov>();
    assert!(m.is_ok());
}

#[test]
#[allow(non_snake_case)]
fn parse_move_Nxf3() {
    let m = "Nxf3".parse::<ParsedMov>();
    assert!(m.is_ok());
}

#[test]
fn parse_move_castle() {
    match "O-O-O".parse().unwrap() {
        ParsedMov::CastleQueen => assert!(true),
        _ => assert!(false, "Castle queen"),
    };
    match "O-O".parse().unwrap() {
        ParsedMov::CastleKing => assert!(true),
        _ => assert!(false, "Castle king"),
    };
}