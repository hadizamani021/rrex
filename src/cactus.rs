use crate::item::Item;

const FIRST_START_Y: usize = 18;
const FIRST_START_X: usize = 200;

pub struct Cactus {
    pub item: Item,
}
impl Cactus {
    pub fn new() -> Self {
        let xs = vec![0, 1, 2];
        let ys = vec![0, 0, 0];
        let representor = '@';
        return Self {
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
    pub fn update(&mut self) {
        // todo: solve overflow problem
        if self.item.start_x == 0 {
            self.item.representor = ' ';
            return;
        }
        self.item.start_x = self.item.start_x - 1;
    }
}
