use crate::{item::Item, movement::MovementState};

const FIRST_START_Y: usize = 10;
const FIRST_START_X: usize = 5;
const MAX_TIME_OF_JUMP: usize = 6;

pub struct Corcodile {
    pub item: Item,
    state: MovementState,
}
impl Corcodile {
    pub fn new() -> Self {
        let xs = vec![6, 7, 6, 7, 4, 4, 4, 4, 5, 5, 5, 5, 3, 4, 4];
        let ys = vec![5, 5, 7, 7, 5, 6, 7, 8, 5, 6, 7, 8, 8, 8, 8];
        let representor = '%';
        return Self {
            state: MovementState::FREE,
            item: Item {
                time_of_last_event: 0,
                xs,
                ys,
                representor,
                start_x: FIRST_START_X,
                start_y: FIRST_START_Y,
            },
        };
    }
    pub fn jump(&mut self) {
        self.state = MovementState::JUMP;
    }
    pub fn update(&mut self) {
        if matches!(self.state, MovementState::JUMP) {
            if self.item.time_of_last_event < MAX_TIME_OF_JUMP / 2 {
                self.item.start_y = self.item.start_y - 1;
                self.item.time_of_last_event = self.item.time_of_last_event + 1;
            } else if self.item.time_of_last_event < MAX_TIME_OF_JUMP {
                self.item.start_y = self.item.start_y + 1;
                self.item.time_of_last_event = self.item.time_of_last_event + 1;
            } else {
                self.item.start_y = FIRST_START_Y;
                self.item.time_of_last_event = 0;
                self.state = MovementState::FREE;
            }
        }
    }
}
