pub struct Item {
    pub start_x: usize,
    pub start_y: usize,
    pub xs: Vec<usize>,
    pub ys: Vec<usize>,
    pub representor: char,
    pub time_of_last_event: usize,
}
impl Item {
    pub fn number_of_points(&self) -> usize {
        return self.xs.len();
    }
    pub fn get_point(&self, index: usize) -> (usize, usize) {
        let x = self.start_x + self.xs[index];
        let y = self.start_y + self.ys[index];
        (x, y)
    }
}
