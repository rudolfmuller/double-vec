#[derive(Debug, Default, Clone)]
pub struct Index {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug, Default, Clone)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Default, Clone)]
pub struct DubleVec<T> {
    pub scale: Size,
    vector: Vec<T>,
}

impl<T: Default + Clone> DubleVec<T> {
    pub fn new(scale: Size) -> Self {
        let vector = vec![T::default(); scale.w * scale.h];
        Self { scale, vector }
    }

    pub fn map(&self, index: Index) -> usize {
        index.y * self.scale.w + index.x
    }

    pub fn push(&mut self, item: T, index: Index) {
        let idx = self.map(index);
        if idx < self.vector.len() {
            self.vector[idx] = item;
        }
    }

    pub fn remove(&mut self, index: Index) {
        let idx = self.map(index);
        if idx < self.vector.len() {
            self.vector[idx] = T::default();
        }
    }

    pub fn reverse(&mut self) {
        self.vector.reverse();
    }

    pub fn get(&self, index: Index) -> Option<&T> {
        let idx = self.map(index);
        self.vector.get(idx)
    }
    pub fn get_vec(&self) -> Vec<T> {
        self.vector.clone()
    }
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        let idx = self.map(index);
        self.vector.get_mut(idx)
    }

    pub fn is_empty(&self) -> bool {
        self.vector.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }
}

impl<T> IntoIterator for DubleVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vector.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DubleVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vector.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DubleVec<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vector.iter_mut()
    }
}

impl<T: std::fmt::Display> DubleVec<T> {
    pub fn print(&self) {
        for y in 0..self.scale.h {
            for x in 0..self.scale.w {
                let index = Index { y, x };
                let idx = index.y * self.scale.w + index.x;
                print!("{} ", self.vector[idx]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: DubleVec<i32> = DubleVec::new(Size { w: 6, h: 5 });
        vec.push(5, Index { x: 1, y: 1 });
        if let Some(value) = vec.get(Index { x: 1, y: 1 }) {
            println!("Value: {}", value);
        } else {
            println!("No value at this index");
        }

        println!("{}", vec.len());
        vec.print();
    }
}

