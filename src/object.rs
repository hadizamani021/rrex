pub struct Object<'a> {
    start_x: u16,
    start_y: u16,
    xs: &'a [u16],
    ys: &'a [u16],
    pub representor: char,
}
impl Object<'_> {
    pub fn corcodile() -> Self {
        let xs = &[6, 7, 6, 7, 4, 4, 4, 4, 5, 5, 5, 5, 3, 4, 4];
        let ys = &[5, 5, 7, 7, 5, 6, 7, 8, 5, 6, 7, 8, 8, 8, 8];
        let representor = '%';
        let start_x = 5;
        let start_y = 0;
        return Self {
            xs,
            ys,
            representor,
            start_x,
            start_y,
        };
    }
    pub fn number_of_points(&self) -> usize {
        return self.xs.len();
    }
    pub fn get_point(&self, index: usize) -> (u16, u16) {
        (self.start_x + self.xs[index], self.start_y + self.ys[index])
    }
}
