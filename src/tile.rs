use piece::Piece;

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Occupied(Piece),
}
