mod play;
mod parse;

use game::Game;
use board::{Board, Item};
use piece::PieceType;
use color::Color;

#[test]
fn init_fen() {
    let g = Game::new();
    assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", g.to_fen());
}

#[test]
fn whites() {
    let b = Board::new();
    let mut pieces = b.pieces(Color::White);
    match pieces.next() {
        Some(Item {piece: PieceType::Rook, index: 0}) => {},
        _ => assert!(false, "whites 1"),
    };
    match pieces.next() {
        Some(Item {piece: PieceType::Knight, index: 1}) => {},
        _ => assert!(false, "whites 2"),
    };
    match pieces.next() {
        Some(Item {piece: PieceType::Bishop, index: 2}) => {},
        _ => assert!(false, "whites 3"),
    };
    match pieces.next() {
        Some(Item {piece: PieceType::Queen, index: 3}) => {},
        _ => assert!(false, "whites 4"),
    };
    match pieces.last() {
        Some(Item {piece: PieceType::Pawn, index: 15}) => {},
        _ => assert!(false, "whites last"),
    };
}

#[test]
fn blacks() {
    let b = Board::new();
    let mut pieces = b.pieces(Color::Black);
    match pieces.next() {
        Some(Item {piece: PieceType::Rook, index: 63}) => {},
        _ => assert!(false, "blacks 1"),
    };
    match pieces.next() {
        Some(Item {piece: PieceType::Knight, index: 62}) => {},
        _ => assert!(false, "blacks 1"),
    };
    match pieces.next() {
        Some(Item {piece: PieceType::Bishop, index: 61}) => {},
        _ => assert!(false, "blacks 2"),
    };
    match pieces.next() {
        Some(Item {piece: PieceType::King, index: 60}) => {},
        _ => assert!(false, "blacks 3"),
    };
    match pieces.last() {
        Some(Item {piece: PieceType::Pawn, index: 48}) => {},
        _ => assert!(false, "blacks last"),
    };
}

#[test]
fn size_of() {
    use std::mem::size_of;
    use color::Color;
    use piece::Piece;
    use tile::Tile;
    use mov::Mov;
    use parse::mov::Mov as ParsedMov;
    use position::Position;
    assert_eq!(1, size_of::<PieceType>());
    assert_eq!(1, size_of::<Color>());
    assert_eq!(2, size_of::<Piece>());
    assert_eq!(3, size_of::<Tile>());
    assert_eq!(24, size_of::<Mov>());
    assert_eq!(32, size_of::<ParsedMov>());
    assert_eq!(192, size_of::<Board>());
    assert_eq!(216, size_of::<Position>());
    assert_eq!(344, size_of::<Game>());
}
