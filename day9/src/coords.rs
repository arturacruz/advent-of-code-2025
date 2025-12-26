pub struct Coord {
    pub x: i32,
    pub y: i32
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
        ((self.x - other.x).unsigned_abs() as usize + 1) * ((self.y - other.y).unsigned_abs() as usize + 1)
    }
}
