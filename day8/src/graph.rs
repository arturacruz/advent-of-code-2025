use std::collections::HashMap;

use crate::coords::Coordinates;

pub struct Graph<'a> {
    coords: HashMap<Coordinates, &'a Coordinates>
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Self { coords: HashMap::new()  }
    } 

    pub fn add(&mut self, key: &'a Coordinates, val: &'a Coordinates) {
        let parent1 = self.find(key);
        let parent2 = self.find(val);

        if let Some(x) = parent1 && let Some(y) = parent2 {
            self.unite(x, y);
        } else if parent1.is_some() {
            self.coords.insert(val.clone(), key);
        } else if parent2.is_some() {
            self.coords.insert(key.clone(), val);
        } else {
            self.coords.insert(key.clone(), key);
            self.coords.insert(val.clone(), key);
        }
    }

    fn parent(&self, val: &'a Coordinates) -> Option<&'a Coordinates> {
        let res = self.coords.get(val)?;
        if *res == val {
            Some(val)
        } else {
            self.parent(res)
        }
    }

    pub fn find(&self, coord: &'a Coordinates) -> Option<&'a Coordinates> {
        self.parent(coord)
    }

    pub fn unite(&mut self, new: &'a Coordinates, old: &'a Coordinates) {
        self.coords.insert(old.clone(), new);
    }

    pub fn get_biggest(&self) -> usize {
        let mut sizes: HashMap<&Coordinates, usize> = HashMap::new();
        
        for key in self.coords.keys() {
            let parent = self.find(key).unwrap();
            if let Some(size) = sizes.get(parent) {
                sizes.insert(parent, size + 1);
            } else {
                sizes.insert(parent, 1);
            }
        }
        
        let mut sizes: Vec<_> = sizes.iter().collect();

        sizes.sort_by(|s, o| s.1.cmp(o.1));
        sizes.reverse();

        sizes.iter().take(3).map(|item| item.1).product::<usize>()
    }
}
