use crate::corcodile::Corcodile;
use crate::gui::Showable;
pub struct Engine<'a> {
    gui: &'a mut (dyn Showable + 'a),
    corcodile: Corcodile<'a>,
    // objects: Vec<Object<'a>>,
}
impl<'a> Engine<'a> {
    pub fn new<T: Showable>(gui: &'a mut T) -> Self {
        let corcodile = Corcodile::new();
        Self { gui, corcodile }
    }
    pub fn show(&mut self) {
        self.gui.show(&self.corcodile);
    }
}
