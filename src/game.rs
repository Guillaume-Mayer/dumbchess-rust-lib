use mov::Mov;
use position::Position;

struct Info {
    event: String,
    site: String,
    date: String,
    round: String,
    white: String,
    black: String,
    result: String,
}

impl Info {
    fn new() -> Info {
        Info {
            event: String::from("?"),
            site: String::from("?"),
            date: String::from("????.??.??"),
            round: String::from("?"),
            white: String::from("You"),
            black: String::from("Me"),
            result: String::from("*"),
        }
    }
    fn to_str(&self) -> String {
        format!("[Event \"{}\"]\n\
                 [Site \"{}\"]\n\
                 [Date \"{}\"]\n\
                 [Round \"{}\"]\n\
                 [White \"{}\"]\n\
                 [Black \"{}\"]\n\
                 [Result \"{}\"]\n", self.event, self.site, self.date, self.round, self.white, self.black, self.result)
    }
}

pub struct Game {
    info: Info,
    history: Vec<Mov>,
    position: Position,
}

impl Game {

    pub fn new() -> Game {
        Game {
            info: Info::new(),
            history: Vec::new(),
            position: Position::new(),
        }
    }

    pub fn play(&mut self, s: &str) {
        match self.position.move_from_str(s) {
            Err(e) => println!("ERROR: {:?}", e),
            Ok(m) => self.play_move(m),
        }
    }

    fn play_move(&mut self, m: Mov) {
        self.position = self.position.play(&m);
        self.history.push(m);
    }

    pub fn to_fen(&self) -> String {
        format!("{} {}", self.position.to_fen(), self.history.len() / 2 + 1)
    }

    pub fn to_pgn(&self) -> String {
        let mut s = self.info.to_str();
        s.push('\n');
        let mut p = Position::new();
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

    pub fn to_str(&self) -> String {
        self.position.to_str()
    }
}
