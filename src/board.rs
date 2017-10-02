use tile::Tile;
use piece::{Piece, PieceType};
use color::Color;

pub struct Board {
    tiles: [Tile; 64],
}

impl Board {
    pub fn new() -> Board {
        let ee = Tile::Empty;
        let wr = Tile::Occupied(Piece::new(Color::White, PieceType::Rook));
        let wn = Tile::Occupied(Piece::new(Color::White, PieceType::Knight));
        let wb = Tile::Occupied(Piece::new(Color::White, PieceType::Bishop));
        let wq = Tile::Occupied(Piece::new(Color::White, PieceType::Queen));
        let wk = Tile::Occupied(Piece::new(Color::White, PieceType::King));
        let wp = Tile::Occupied(Piece::new(Color::White, PieceType::Pawn));
        let br = Tile::Occupied(Piece::new(Color::Black, PieceType::Rook));
        let bn = Tile::Occupied(Piece::new(Color::Black, PieceType::Knight));
        let bb = Tile::Occupied(Piece::new(Color::Black, PieceType::Bishop));
        let bq = Tile::Occupied(Piece::new(Color::Black, PieceType::Queen));
        let bk = Tile::Occupied(Piece::new(Color::Black, PieceType::King));
        let bp = Tile::Occupied(Piece::new(Color::Black, PieceType::Pawn));
        Board {
            tiles: [
                wr, wn, wb, wq, wk, wb, wn, wr,
                wp, wp, wp, wp, wp, wp, wp, wp,
                ee, ee, ee, ee, ee, ee, ee, ee,
                ee, ee, ee, ee, ee, ee, ee, ee,
                ee, ee, ee, ee, ee, ee, ee, ee,
                ee, ee, ee, ee, ee, ee, ee, ee,
                bp, bp, bp, bp, bp, bp, bp, bp,
                br, bn, bb, bq, bk, bb, bn, br,
            ]
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut tiles: [Tile; 64] = [Tile::Empty; 64];
        for i in 0..64 {
            tiles[i] = self.tiles[i];
        }
        Board {tiles}
    }
}