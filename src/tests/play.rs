use game::Game;

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
#[allow(non_snake_case)]
fn play_e4_c5_Nf3_fen() {
    let mut g = Game::new();
    g.play("e4");
    g.play("c5");
    g.play("Nf3");
    assert_eq!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2", g.to_fen());
}

#[test]
#[allow(non_snake_case)]
fn play_e4_c5_Nf3_pgn() {
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
fn play_e2_e3_fen() {
    let mut g = Game::new();
    g.play("e2-e3");
    assert_eq!("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1", g.to_fen());
}

#[test]
fn play_en_passant_fen() {
    let mut g = Game::new();
    g.play("e4");
    g.play("Nb8-c6");
    g.play("e4-e5");
    g.play("d5");
    g.play("e5xd6");
    println!("{}", g.to_pgn());
    assert_eq!("r1bqkbnr/ppp1pppp/2nP4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3", g.to_fen());
}

#[test]
fn play_en_passant_pgn() {
    let mut g = Game::new();
    g.play("e4");
    g.play("Nb8-c6");
    g.play("e4-e5");
    g.play("d5");
    g.play("e5xd6");
    assert_eq!("[Event \"?\"]\n\
                [Site \"?\"]\n\
                [Date \"????.??.??\"]\n\
                [Round \"?\"]\n\
                [White \"You\"]\n\
                [Black \"Me\"]\n\
                [Result \"*\"]\n\n\
                1.e4 Nc6 2.e5 d5 3.exd6 ", g.to_pgn());
}