use mov::Mov;
use position::Position;

pub struct Game {
    history: Vec<Mov>,
    position: Position,
}

impl Game {
    pub fn new() -> Game {
        Game {
            history: Vec::new(),
            position: Position::new(),
        }
    }
    pub fn play(&mut self, m: Mov) {
        //self.position = self.position.play(m);
        self.history.push(m);
    }
}
