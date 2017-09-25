enum Color {
    White,
    Black,
}

enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

struct Piece {
    color: Color,
    piece: PieceType,
}

impl Piece {
    fn to_char(&self) -> char {
        use self::PieceType::*;
        match self.color {
            Color::White => match self.piece {
                King => 'K',
                Queen => 'Q',
                Rook => 'R',
                Bishop => 'B',
                Knight => 'N',
                Pawn => 'P',
            },
            Color::Black => match self.piece {
                King => 'k',
                Queen => 'q',
                Rook => 'r',
                Bishop => 'b',
                Knight => 'n',
                Pawn => 'p',
            }, 
        }
    }
}

enum Tile {
    Out,
    Empty,
    Occupied(Piece),
}

impl Tile {
    fn to_char(&self) -> char {
        match *self {
            Tile::Out => ' ',
            Tile::Empty => '.',
            Tile::Occupied(ref piece) => piece.to_char(),
        }
    }
}

pub struct Board {
    tiles: [Tile; 120]
}

impl Board {
    pub fn new() -> Board {
        const o: Tile = Tile::Out;
        const e: Tile = Tile::Empty;
        const R: Tile = Tile::Occupied(Piece {color: Color::White, piece: PieceType::Rook});
        const N: Tile = Tile::Occupied(Piece {color: Color::White, piece: PieceType::Knight});
        const B: Tile = Tile::Occupied(Piece {color: Color::White, piece: PieceType::Bishop});
        const Q: Tile = Tile::Occupied(Piece {color: Color::White, piece: PieceType::Queen});
        const K: Tile = Tile::Occupied(Piece {color: Color::White, piece: PieceType::King});
        const P: Tile = Tile::Occupied(Piece {color: Color::White, piece: PieceType::Pawn});
        const r: Tile = Tile::Occupied(Piece {color: Color::Black, piece: PieceType::Rook});
        const n: Tile = Tile::Occupied(Piece {color: Color::Black, piece: PieceType::Knight});
        const b: Tile = Tile::Occupied(Piece {color: Color::Black, piece: PieceType::Bishop});
        const q: Tile = Tile::Occupied(Piece {color: Color::Black, piece: PieceType::Queen});
        const k: Tile = Tile::Occupied(Piece {color: Color::Black, piece: PieceType::King});
        const p: Tile = Tile::Occupied(Piece {color: Color::Black, piece: PieceType::Pawn});
        Board {
            tiles: [
                o, o, o, o, o, o, o, o, o, o,
                o, o, o, o, o, o, o, o, o, o,
                o, R, N, B, Q, K, B, N, R, o,
                o, P, P, P, P, P, P, P, P, o,
                o, e, e, e, e, e, e, e, e, o,
                o, e, e, e, e, e, e, e, e, o,
                o, e, e, e, e, e, e, e, e, o,
                o, e, e, e, e, e, e, e, e, o,
                o, p, p, p, p, p, p, p, p, o,
                o, r, n, b, q, k, b, n, r, o,
                o, o, o, o, o, o, o, o, o, o,
                o, o, o, o, o, o, o, o, o, o,
            ]
        }
    }

    pub fn to_str(&self) -> String {
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