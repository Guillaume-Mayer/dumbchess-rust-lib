use std::char;
use std::fmt;

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

    pub fn to_str(&self) -> String {
        let mut s = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                s.push(self.tiles[rank * 8 + file].to_char());
                if file < 7 { s.push(' ') };
            }
            if rank > 0 { s.push('\n'); }
        }
        s
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

    pub fn pieces(&self, color: Color) -> Pieces {
        Pieces::new(color, self.tiles)
    }

}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {tiles: self.tiles}
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..8 {
            let i = r * 8;
            write!(f, "\n[{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]\n", self.tiles[i], self.tiles[i + 1], self.tiles[i + 2], self.tiles[i + 3], self.tiles[i + 4], self.tiles[i + 5], self.tiles[i + 6], self.tiles[i + 7])?;
        }
        Ok(())
    }
}

pub struct Item {
    pub piece: PieceType,
    pub index: usize,
}

pub struct Pieces {
    color: Color,
    tiles: [Tile; 64],
    index: usize,
    done: bool,
    found: u8,
}

impl Pieces {

    fn new(color: Color, tiles: [Tile; 64]) -> Pieces {
        Pieces {
            color,
            tiles,
            index: if color == Color::White {0} else {63},
            done: false,
            found: 0,
        }
    }

    fn step(&mut self) -> bool {
        match self.color {
            Color::White => {
                if self.index == 63 {
                    self.done = true;
                } else {
                    self.index += 1;
                }
            },
            Color::Black => {
                if self.index == 0 {
                    self.done = true;
                } else {
                    self.index -= 1;
                }
            },
        }
        self.done
    }

    fn found(&mut self, p: PieceType) -> Option<Item> {
        self.found += 1;
        if self.found == 16 {
            self.done = true;
        }
        Some(Item {piece: p, index: self.index})
    }
}

impl Iterator for Pieces {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut next = None;
        while let None = next {
            match self.tiles[self.index] {
                Tile::Occupied(p) if p.color() == self.color => {
                    next = self.found(p.piece());
                },
                _ => {},
            };
            if self.step() {
                break;
            }
        }
        next
    }
}