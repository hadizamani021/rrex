use crate::corcodile::Corcodile;
use crate::gui::Showable;
pub enum GameEvent {
    JumpPlayer,
    Exit,
}
pub struct Engine<'a> {
    gui: &'a mut (dyn Showable + 'a),
    corcodile: Corcodile<'a>,
}
impl<'a> Engine<'a> {
    pub fn new<T: Showable>(gui: &'a mut T) -> Self {
        let corcodile = Corcodile::new();
        Self { gui, corcodile }
    }
    pub fn run(&mut self) {
        self.corcodile.update();
        self.gui.show(&self.corcodile);
    }
    pub fn on_event(&mut self, event: GameEvent) {
        if matches!(event, GameEvent::JumpPlayer) {
            self.corcodile.jump();
        }
    }
}
