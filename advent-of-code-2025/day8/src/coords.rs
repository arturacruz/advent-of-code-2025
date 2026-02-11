use std::fmt::Debug;

use crate::distance::Distance;

#[derive(PartialEq, Hash, Eq, Clone)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
    pub z: isize
}

impl Coordinates {
    pub fn dist_to(&self, other: &Coordinates) -> Distance {
        let x = (other.x - self.x).pow(2);
        let y = (other.y - self.y).pow(2);
        let z = (other.z - self.z).pow(2);
        let dist = (x + y + z) as f64;
        Distance::new(dist, (self.clone(), other.clone()))
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl<'a> FromIterator<&'a str> for Coordinates {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().expect("Coords missing X");
        let y = iter.next().expect("Coords missing Y");
        let z = iter.next().expect("Coords missing Z");
        let (x, y, z) = (
            x.parse().expect("Failed to convert str to u64"),
            y.parse().expect("Failed to convert str to u64"),
            z.parse().expect("Failed to convert str to u64"),
        );
        Self { x, y, z }
    }
}
