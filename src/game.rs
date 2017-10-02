use mov::Mov;
use position::Position;
use super::Play;

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

    pub fn play(&mut self, s: &'static str) {
        let m = Position::move_from_str(s);
        self.play_move(m);
    }

    fn play_move(&mut self, m: Mov) {
        self.position = self.position.play(&m);
        self.history.push(m);
    }

    pub fn to_fen(&self) -> String {
        format!("{} {}", self.position.to_fen(), self.history.len() / 2 + 1)
    }

}
