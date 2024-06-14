use std::io::{self, Write};

use crossterm::{cursor, style, style::Colorize, terminal, ExecutableCommand, QueueableCommand};

use crate::{gui::Showable, objects::Object};

pub struct Terminal {
    stdout: io::Stdout,
}
impl Terminal {
    pub fn new() -> Self {
        let stdout = io::stdout();
        return Self { stdout };
    }
}
impl Showable for Terminal {
    fn show(&mut self, objects: &Vec<Object>) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        for object in objects.iter() {
            for i in 0..object.number_of_points() {
                let point = object.get_point(i);
                self.stdout
                    .queue(cursor::MoveTo(point.0, point.1))
                    .unwrap()
                    .queue(style::PrintStyledContent(object.representor.magenta()))
                    .unwrap();
            }
        }
        self.stdout.flush().unwrap();
    }
}
