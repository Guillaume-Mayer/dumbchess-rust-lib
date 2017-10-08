//mod play;
mod parse;

use game::Game;

#[test]
fn init_fen() {
    let g = Game::new();
    assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", g.to_fen());
}
