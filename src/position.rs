use board::Board;
use color::Color;

#[derive(Debug)]
pub struct Position {
    board: Board,
    color_to_play: Color,
}

impl Position {
    pub fn new() -> Position {
        Position {
            board: Board::new(),
            color_to_play: Color::White,
        }
    }
}