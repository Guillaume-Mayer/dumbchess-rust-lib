use piece::Piece;

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Occupied(Piece),
}

impl Tile {
    pub fn to_char(&self) -> char {
        match *self {
            Tile::Empty => '.',
            Tile::Occupied(p) => p.to_char(),
        }
    }
}
