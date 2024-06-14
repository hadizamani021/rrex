use crate::{gui::Showable, object::Object};

pub struct Engine<'a> {
    gui: &'a mut (dyn Showable + 'a),
    objects: Vec<Object<'a>>,
}
impl<'a> Engine<'a> {
    pub fn new<T: Showable>(gui: &'a mut T) -> Self {
        let corcodile = Object::corcodile();
        let mut objects = Vec::new();
        objects.push(corcodile);
        Self { gui, objects }
    }
    pub fn show(&mut self) {
        self.gui.show(&self.objects);
    }
}
