use crate::coords::Coordinates;

#[derive(Debug)]
pub struct Distance {
    pub value: f64,
    pub coords: (Coordinates, Coordinates)
}

impl Distance {
    pub fn new(value: f64, coords: (Coordinates, Coordinates)) -> Self {
        Self { value, coords }
    }
}
