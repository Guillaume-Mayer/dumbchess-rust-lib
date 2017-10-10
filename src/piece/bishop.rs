struct Bishop;

impl MyPiece for Bishop {
    fn to_fen(&self) -> char {
        'B'
    }
}