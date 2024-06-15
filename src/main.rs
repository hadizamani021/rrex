use std::{thread::sleep, time::Duration};

mod corcodile;
mod engine;
mod gui;
mod item;
mod movement;
mod terminal;
use crossterm::event::{poll, read, Event, KeyCode};
use engine::GameEvent;

pub fn handle_pressed_keys() -> Option<engine::GameEvent> {
    if poll(Duration::from_millis(10)).unwrap() {
        let key = read().unwrap();

        while poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
        }

        return match key {
            Event::Key(event) => match event.code {
                KeyCode::Char('w') | KeyCode::Up => Some(engine::GameEvent::JumpPlayer),
                KeyCode::Char('q') => Some(GameEvent::Exit),
                _ => None,
            },
            _ => None,
        };
    }
    println!("here");
    None
}
fn main() {
    let mut terminal = terminal::Terminal::new();
    let mut engine = engine::Engine::new(&mut terminal);
    loop {
        match handle_pressed_keys() {
            Some(engine::GameEvent::JumpPlayer) => {
                engine.on_event(engine::GameEvent::JumpPlayer);
                engine.run();
                sleep(Duration::from_millis(50));
            }
            Some(engine::GameEvent::Exit) => {
                std::process::exit(1);
            }
            _ => {
                engine.run();
                sleep(Duration::from_millis(50));
            }
        }
    }
}
