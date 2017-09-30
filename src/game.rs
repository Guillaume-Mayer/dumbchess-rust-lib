use mov::Mov;
use position::Position;

#[derive(Debug)]
pub struct Game {
    history: Vec<Mov>,
    position: Position,
    half_move_clock: u16,
}

impl Game {

    pub fn new() -> Game {
        Game {
            history: Vec::new(),
            position: Position::new(),
            half_move_clock: 0,
        }
    }

}
