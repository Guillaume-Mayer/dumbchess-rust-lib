use super::board::Board;

#[test]
fn board_to_str() {
    let b = Board::new();
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