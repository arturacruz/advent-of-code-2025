use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position { 
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };

    pub fn new(pos: (i32, i32)) -> Self {
        let (x, y) = pos;
        Self { x, y }
    }

    pub fn get(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (x, y) = self.get();
        let (newx, newy) = rhs.get();

        let pos = (x + newx, y + newy);
        Self::new(pos)
    }
}
