use crate::item::Item;

pub struct Corcodile<'a> {
    pub item: Item<'a>,
}

impl Corcodile<'_> {
    pub fn new() -> Self {
        let xs = &[6, 7, 6, 7, 4, 4, 4, 4, 5, 5, 5, 5, 3, 4, 4];
        let ys = &[5, 5, 7, 7, 5, 6, 7, 8, 5, 6, 7, 8, 8, 8, 8];
        let representor = '%';
        let start_x = 5;
        let start_y = 0;
        return Self {
            item: Item {
                xs,
                ys,
                representor,
                start_x,
                start_y,
            },
        };
    }
}
