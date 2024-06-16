use crate::{cactus::Cactus, corcodile::Corcodile, ground::Ground};

pub trait Showable {
    // todo: use many cactuses
    fn show(&mut self, corcodile: &Corcodile, ground: &Ground, cactus: &Cactus);
}
