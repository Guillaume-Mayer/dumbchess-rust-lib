use tile::Tile;
use piece::{Piece, PieceType};
use color::Color;

pub struct Board {
    tiles: [Tile; 120]
}

impl Board {
    pub fn new() -> Board {
        let oo = Tile::Out;
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
                oo, oo, oo, oo, oo, oo, oo, oo, oo, oo,
                oo, oo, oo, oo, oo, oo, oo, oo, oo, oo,
                oo, wr, wn, wb, wq, wk, wb, wn, wr, oo,
                oo, wp, wp, wp, wp, wp, wp, wp, wp, oo,
                oo, ee, ee, ee, ee, ee, ee, ee, ee, oo,
                oo, ee, ee, ee, ee, ee, ee, ee, ee, oo,
                oo, ee, ee, ee, ee, ee, ee, ee, ee, oo,
                oo, ee, ee, ee, ee, ee, ee, ee, ee, oo,
                oo, bp, bp, bp, bp, bp, bp, bp, bp, oo,
                oo, br, bn, bb, bq, bk, bb, bn, br, oo,
                oo, oo, oo, oo, oo, oo, oo, oo, oo, oo,
                oo, oo, oo, oo, oo, oo, oo, oo, oo, oo,
            ]
        }
    }

    fn to_str(&self) -> String {
        let mut s = String::new();
        for row in (2..10).rev() {
            for col in 1..9 {
                s.push(self.tiles[row * 10 + col].to_char());
            }
            s.push('\n');
        }
        s
    }
}

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
