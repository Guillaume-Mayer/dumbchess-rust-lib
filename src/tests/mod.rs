#[test]
fn board_to_str() {
    let b = super::board::Board::new();
    assert_eq!(
        "rnbqkbnr\n\
        pppppppp\n\
        ........\n\
        ........\n\
        ........\n\
        ........\n\
        PPPPPPPP\n\
        RNBQKBNR\n", format!("{}", b));
}