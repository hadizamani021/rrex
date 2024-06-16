use crate::item::Item;

const FIRST_START_Y: usize = 19;
const FIRST_START_X: usize = 0;
const LEN_OF_GROUND: usize = 1000;

pub struct Ground {
    pub item: Item,
}
impl Ground {
    pub fn new() -> Self {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for i in 0..LEN_OF_GROUND {
            xs.push(i);
            ys.push(0);
        }
        let representor = '_';
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
}
