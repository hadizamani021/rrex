use std::io::{self, Write};

use crate::cactus::Cactus;
use crate::ground::Ground;
use crate::{corcodile::Corcodile, gui::Showable};
use crossterm::style::Stylize;
use crossterm::terminal::enable_raw_mode;
use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};

pub struct Terminal {
    stdout: io::Stdout,
}
impl Terminal {
    pub fn new() -> Self {
        let stdout = io::stdout();
        enable_raw_mode().unwrap();
        return Self { stdout };
    }
}
impl Showable for Terminal {
    fn show(&mut self, corcodile: &Corcodile, ground: &Ground, cactuses: &Vec<Cactus>) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        let object = &corcodile.item;
        for i in 0..object.number_of_points() {
            let point = object.get_point(i);
            self.stdout
                .queue(cursor::MoveTo(
                    point.0.try_into().unwrap(),
                    point.1.try_into().unwrap(),
                ))
                .unwrap()
                .queue(style::PrintStyledContent(object.representor.magenta()))
                .unwrap();
        }
        //remove duplicate code
        let object = &ground.item;
        for i in 0..object.number_of_points() {
            let point = object.get_point(i);
            self.stdout
                .queue(cursor::MoveTo(
                    point.0.try_into().unwrap(),
                    point.1.try_into().unwrap(),
                ))
                .unwrap()
                .queue(style::PrintStyledContent(object.representor.magenta()))
                .unwrap();
        }
        //remove duplicate code
        for cactus in cactuses {
            let object = &cactus.item;
            for i in 0..object.number_of_points() {
                let point = object.get_point(i);
                self.stdout
                    .queue(cursor::MoveTo(
                        point.0.try_into().unwrap(),
                        point.1.try_into().unwrap(),
                    ))
                    .unwrap()
                    .queue(style::PrintStyledContent(object.representor.magenta()))
                    .unwrap();
            }
        }
        self.stdout.flush().unwrap();
    }
}
