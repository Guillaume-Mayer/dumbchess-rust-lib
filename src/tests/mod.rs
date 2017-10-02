use super::game::Game;

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