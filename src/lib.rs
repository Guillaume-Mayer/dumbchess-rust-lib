mod color;
mod piece;
mod tile;
mod board;
mod position;
mod mov;

trait Play {
    fn play(&self, m: &mov::Mov) -> Self;
}

pub mod game;

#[cfg(test)]
mod tests;

