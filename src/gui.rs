use crate::{corcodile::Corcodile, ground::Ground};

pub trait Showable {
    fn show(&mut self, corcodile: &Corcodile, ground: &Ground);
}
