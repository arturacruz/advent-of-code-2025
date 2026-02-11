pub struct Coord {
    pub x: usize,
    pub y: usize
}

impl Coord {
    pub fn new(x: &str, y: &str) -> Self {
        let (x, y) = (
            x.parse().expect("X not a number"),
            y.parse().expect("Y not a number")
        );
        Self { x, y }
    }

    pub fn area(&self, other: &Self) -> usize {
        ((self.x as i32 - other.x as i32).unsigned_abs() as usize + 1) * 
            ((self.y as i32 - other.y as i32).unsigned_abs() as usize + 1)
    }
}
