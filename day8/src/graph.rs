use std::collections::HashMap;

use crate::coords::Coordinates;

pub struct Graph<'a> {
    coords: HashMap<Coordinates, &'a Coordinates>,
    len: usize
}

impl<'a> Graph<'a> {
    pub fn new(len: usize) -> Self {
        Self { 
            coords: HashMap::new(),
            len
        }
    } 

    pub fn add(&mut self, key: &'a Coordinates, val: &'a Coordinates) -> bool {
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

        self.count()
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

    fn count(&self) -> bool {

        let mut sizes: HashMap<&Coordinates, usize> = HashMap::new();
        
        for key in self.coords.keys() {
            let parent = self.find(key).unwrap();
            if let Some(size) = sizes.get(parent) {
                sizes.insert(parent, size + 1);
            } else {
                sizes.insert(parent, 1);
            }
            if sizes.len() > 1 {
                return false;
            }
        }       
        *sizes.values().next().unwrap() == self.len
    }

}
