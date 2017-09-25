pub mod chess;

#[cfg(test)]
mod tests {

    #[test]
    fn board_to_str() {
        let b = super::chess::Board::new();
        assert_eq!(
           "rnbqkbnr\n\
            pppppppp\n\
            ........\n\
            ........\n\
            ........\n\
            ........\n\
            PPPPPPPP\n\
            RNBQKBNR\n", b.to_str());
    }
}

