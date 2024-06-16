use crate::corcodile::Corcodile;
use crate::ground::Ground;
use crate::gui::Showable;
pub enum GameEvent {
    JumpPlayer,
    Exit,
}
pub struct Engine<'a> {
    gui: &'a mut (dyn Showable + 'a),
    ground: Ground<'a>,
    corcodile: Corcodile<'a>,
}
impl<'a> Engine<'a> {
    pub fn new<T: Showable>(gui: &'a mut T) -> Self {
        let corcodile = Corcodile::new();
        let ground = Ground::new();
        Self {
            gui,
            corcodile,
            ground,
        }
    }
    pub fn run(&mut self) {
        self.corcodile.update();
        self.gui.show(&self.corcodile, &self.ground);
    }
    pub fn on_event(&mut self, event: GameEvent) {
        if matches!(event, GameEvent::JumpPlayer) {
            self.corcodile.jump();
        }
    }
}
