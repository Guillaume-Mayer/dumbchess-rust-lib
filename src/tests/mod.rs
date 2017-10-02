use super::position::Position;

#[test]
fn pos_to_fen() {
    let p = Position::new();
    assert_eq!("", p.to_fen());
}