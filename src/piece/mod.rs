mod king;
mod queen;
mod rook;
mod bishop;
mod knight;
mod pawn;

pub use self::king::King;
pub use self::queen::Queen;
pub use self::rook::Rook;
pub use self::bishop::Bishop;
pub use self::knight::Knight;
pub use self::pawn::Pawn;

use color::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn to_fen(&self) -> char {
        match *self {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Pawn => 'P',
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    color: Color,
    piece: PieceType,
}

impl Piece {

    pub fn new(color: Color, piece: PieceType) -> Piece {
        Piece {
            color,
            piece,
        }
    }

    pub fn to_fen(&self) -> char {
        match self.color {
            Color::White => self.piece.to_fen(),
            Color::Black => match self.piece {
                PieceType::King => 'k',
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                PieceType::Pawn => 'p',
            },
        }
    }

    pub fn to_char(&self) -> char {
        match self.color {
            Color::White => match self.piece {
                PieceType::King => '♔',
                PieceType::Queen => '♕',
                PieceType::Rook => '♖',
                PieceType::Bishop => '♗',
                PieceType::Knight => '♘',
                PieceType::Pawn => '♙',
            },
            Color::Black => match self.piece {
                PieceType::King => '♚',
                PieceType::Queen => '♛',
                PieceType::Rook => '♜',
                PieceType::Bishop => '♝',
                PieceType::Knight => '♞',
                PieceType::Pawn => '♟',
            },
        }
    }

    pub fn to_san(&self) -> &str {
        match self.piece {
            PieceType::Pawn => "",
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Rook => "R",
            PieceType::Bishop => "B",
            PieceType::Knight => "N",
        }
    }

    pub fn is_pawn(&self) -> bool {
        match self.piece {
            PieceType::Pawn => true,
            _ => false,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn piece(&self) -> PieceType {
        self.piece
    }

}

pub trait _Piece {
    fn to_char(&self) -> char;
    fn to_fen(&self) -> char;
    fn to_san(&self) -> &str;
    fn color(&self) -> Color;
    fn is_pawn(&self) -> bool {
        false
    }
}

enum Tile {
    Empty,
    Occupied(Box<_Piece>),
}

impl Clone for Tile {
    fn clone(&self) -> Tile {
        match *self {
            Tile::Empty => Tile::Empty,
            Tile::Occupied(ref p) => Tile::Occupied(Box::new(p.clone() as _Piece)),
        }
    }
}

struct Board {
    tiles: [Tile; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [
                Tile::Empty,
                Tile::Occupied(Box::new(King::new(Color::White))),
                Tile::Occupied(Box::new(Queen::new(Color::Black))),
            ]
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board { tiles: [
            self.tiles[0].clone(),
            self.tiles[1].clone(),
            self.tiles[2].clone(),
        ] }
    }
}
