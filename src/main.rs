use engine::Engine;
use terminal::Terminal;

mod engine;
mod gui;
mod object;
mod terminal;
fn main() {
    let mut terminal = Terminal::new();
    let mut engine = Engine::new(&mut terminal);
    engine.show();
}
