use crate::objects::Object;

pub trait Showable {
    fn show(&mut self, objects: &Vec<Object>);
}
