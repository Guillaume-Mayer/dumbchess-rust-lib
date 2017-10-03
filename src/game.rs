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

    pub fn play(&mut self, s: &'static str) {
        let m = self.position.move_from_str(s);
        self.play_move(m);
    }

    fn play_move(&mut self, m: Mov) {
        self.position = self.position.play(&m);
        self.history.push(m);
    }

    pub fn to_fen(&self) -> String {
        format!("{} {}", self.position.to_fen(), self.history.len() / 2 + 1)
    }

    pub fn to_pgn(&self) -> String {
        let mut p = Position::new();
        let mut s = String::new();
        let mut i = 0;
        loop {
            let m = self.history.get(i);
            match m {
                None => break,
                Some(m) => {
                    if i % 2 == 0 {
                        s.push_str(&format!("{}.", i / 2 + 1));
                    }
                    s.push_str(&p.move_to_san(&m));
                    s.push(' ');                 
                    i += 1;
                    p = p.play(&m);
                },
            };
        }
        s
    }
}
