pub struct Object<'a> {
    pub start_x: u16,
    pub start_y: u16,
    pub xs: &'a [u16],
    pub ys: &'a [u16],
    pub representor: char,
}
impl Object<'_> {
    pub fn number_of_points(&self) -> usize {
        return self.xs.len();
    }
    pub fn get_point(&self, index: usize) -> (u16, u16) {
        (self.start_x + self.xs[index], self.start_y + self.ys[index])
    }
}
