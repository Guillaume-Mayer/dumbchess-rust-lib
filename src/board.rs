use std::char;

use tile::Tile;
use piece::{Piece, PieceType};
use color::Color;
use mov::Mov;

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

    pub fn tile_at(&self, index: usize) -> Tile {
        self.tiles[index]
    }

    pub fn play(&self, m: &Mov) -> Board {
        let mut board: Board = self.clone();
        match *m {
            Mov::Quiet(t1, t2) | Mov::TwoPush(t1, t2) | Mov::Capture(t1, t2) => {
                board.tiles[t2] = board.tiles[t1];
                board.tiles[t1] = Tile::Empty;
            },
            Mov::EnPassant(t1, t2) => {
                board.tiles[t2] = board.tiles[t1];
                board.tiles[t1] = Tile::Empty;
                if t2 > t1 {
                    board.tiles[t2 - 8] = Tile::Empty;
                } else {
                    board.tiles[t2 + 8] = Tile::Empty;
                }
            },
            Mov::CastleKing(t1) => {
                board.tiles[t1 + 2] = board.tiles[t1];
                board.tiles[t1] = Tile::Empty;
                board.tiles[t1 - 1] = board.tiles[t1 + 3];
                board.tiles[t1 + 3] = Tile::Empty;
            },
            Mov::CastleQueen(t1) => {
                board.tiles[t1 - 2] = board.tiles[t1];
                board.tiles[t1] = Tile::Empty;
                board.tiles[t1 + 1] = board.tiles[t1 - 4];
                board.tiles[t1 - 4] = Tile::Empty;
            },
            Mov::Promotion(t1, t2, p) | Mov::PromotionCapture(t1, t2, p) => {
                let color = match board.tiles[t1] {
                    Tile::Empty => panic!("Empty tile"),
                    Tile::Occupied(p) => p.get_color(),
                };
                board.tiles[t2] = Tile::Occupied(Piece::new(color, p));
                board.tiles[t1] = Tile::Empty;
            },
        }
        board
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
