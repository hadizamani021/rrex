use crate::cactus::Cactus;
use crate::corcodile::Corcodile;
use crate::ground::Ground;
use crate::gui::Showable;
pub enum GameEvent {
    JumpPlayer,
    Exit,
}
pub struct Engine<'a> {
    gui: &'a mut (dyn Showable + 'a),
    ground: Ground,
    corcodile: Corcodile,
    cactuses: Vec<Cactus>,
}
impl<'a> Engine<'a> {
    pub fn new<T: Showable>(gui: &'a mut T) -> Self {
        let corcodile = Corcodile::new();
        let ground = Ground::new();
        let mut cactuses = Vec::new();
        cactuses.push(Cactus::new());
        Self {
            gui,
            corcodile,
            ground,
            cactuses,
        }
    }
    pub fn run(&mut self) {
        self.corcodile.update();
        for cactus in &mut self.cactuses {
            cactus.update();
        }
        self.gui.show(&self.corcodile, &self.ground, &self.cactuses);
    }
    pub fn on_event(&mut self, event: GameEvent) {
        if matches!(event, GameEvent::JumpPlayer) {
            self.corcodile.jump();
        }
    }
}
