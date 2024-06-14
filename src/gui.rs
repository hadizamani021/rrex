use crate::corcodile::Corcodile;

pub trait Showable {
    fn show(&mut self, corcodile: &Corcodile);
}
