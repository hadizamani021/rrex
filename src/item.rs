pub struct Item<'a> {
    pub start_x: usize,
    pub start_y: usize,
    pub xs: &'a [usize],
    pub ys: &'a [usize],
    pub representor: char,
    pub time_of_last_event: usize,
}
impl Item<'_> {
    pub fn number_of_points(&self) -> usize {
        return self.xs.len();
    }
    pub fn get_point(&self, index: usize) -> (usize, usize) {
        (self.start_x + self.xs[index], self.start_y + self.ys[index])
    }
}
