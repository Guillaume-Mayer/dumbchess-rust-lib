use std::char;

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

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                match self.tiles[rank*8 + file] {
                    Tile::Empty => empty += 1,
                    Tile::Occupied(p) => {
                        if empty > 0 {
                            fen.push(char::from_digit(empty, 10).unwrap());
                            empty = 0;
                        }
                        fen.push(p.to_fen());
                        },
                }
            }
            if empty > 0 {
                fen.push(char::from_digit(empty, 10).unwrap());
            }
            if rank > 0 { fen.push('/'); }
        }
        fen
    }

    pub fn tile_at(&self, i: usize) -> Tile {
        self.tiles[i]
    }

    pub fn mov(&mut self, i1: usize, i2: usize) {
        self.tiles[i2] = self.tiles[i1];
        self.tiles[i1] = Tile::Empty;
    }

    pub fn prom(&mut self, i1: usize, i2: usize, p: Piece) {
        self.tiles[i2] = Tile::Occupied(p);
        self.tiles[i1] = Tile::Empty;
    }

    pub fn empty(&mut self, i: usize) {
        self.tiles[i] = Tile::Empty;
    }
}

impl Clone for Board {
    /*fn clone(&self) -> Board {
        let mut tiles: [Tile; 64] = [Tile::Empty; 64];
        for i in 0..64 {
            tiles[i] = self.tiles[i];
        }
        Board {tiles}
    }*/
    fn clone(&self) -> Board {
        Board {tiles: self.tiles}
    }
}
