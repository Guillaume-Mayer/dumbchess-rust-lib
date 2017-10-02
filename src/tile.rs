use piece::Piece;

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Empty,
    Occupied(Piece),
}

impl Tile {
    pub fn to_char(&self) -> char {
        match *self {
            Tile::Empty => '.',
            Tile::Occupied(ref piece) => piece.to_char(),
        }
    }
}