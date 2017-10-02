mod color;
mod piece;
mod tile;
mod board;
mod position;

trait Play {
    fn play(&self, m: &mov::Mov) -> Self;
}

mod mov;
mod game;

#[cfg(test)]
mod tests;

