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
