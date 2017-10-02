use board::Board;
use color::Color;

pub struct Position {
    board: Board,
    color_to_play: Color,
    half_move_clock: u16,
}

impl Position {
    pub fn new() -> Position {
        Position {
            board: Board::new(),
            color_to_play: Color::White,
            half_move_clock: 0,
        }
    }
}

use mov::Mov;
use mov::Play;

impl Play for Position {
    fn play(&self, m: Mov) -> Position {
        Position {
            board: self.board.play(m),
            color_to_play: self.color_to_play.swap(),
            half_move_clock: self.half_move_clock + 1,
        }
    }
    fn play_mut(&mut self, m: Mov) {
        unimplemented!();
    }
}