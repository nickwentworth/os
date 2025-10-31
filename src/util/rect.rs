#[derive(Clone, Copy)]
pub struct Rect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl Rect {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self {
            x1: x1.min(x2),
            y1: y1.min(y2),
            x2: x1.max(x2),
            y2: y1.max(y2),
        }
    }

    pub fn width(self) -> usize {
        self.x2 - self.x1
    }

    pub fn height(self) -> usize {
        self.y2 - self.y1
    }

    pub fn points(self) -> impl Iterator<Item = (usize, usize)> {
        (self.x1..self.x2).flat_map(move |x| (self.y1..self.y2).map(move |y| (x, y)))
    }
}
