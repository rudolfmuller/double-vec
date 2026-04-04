/// The `Position` is a structure composed of variables of type `usize` named `x` and `y`
#[derive(Debug, Default, Clone)]
pub struct Index {
    pub x: usize,
    pub y: usize,
}
/// The `Size` is a structure composed of variables of type `usize` named `w` and `h`
#[derive(Debug, Default, Clone)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Default, Clone)]
pub struct DubleVec<T> {
    /// 2D size of `Size`
    pub scale: Size,
    vector: Vec<T>,
}

impl<T: Default + Clone + PartialEq> DubleVec<T> {
    /// Creates a new `DubleVec`
    pub fn new(scale: Size) -> Self {
        let vector = vec![T::default(); scale.w * scale.h];
        Self { scale, vector }
    }

    fn map(&self, index: Index) -> Option<usize> {
        if index.x < self.scale.w && index.y < self.scale.h {
            Some(index.y * self.scale.w + index.x)
        } else {
            None
        }
    }
    /// Push new item`T` on `index`
    pub fn push(&mut self, item: T, index: Index) {
        if let Some(idx) = self.map(index) {
            if self.vector[idx] == T::default() {
                self.vector[idx] = item;
            }
        }
    }
    /// Force push new item`T` on `index`
    pub fn fush(&mut self, item: T, index: Index) {
        if let Some(idx) = self.map(index) {
            self.vector[idx] = item;
        }
    }
    /// Remove item`T` on `index`
    /// Set item`T` on `index` to default value
    pub fn remove(&mut self, index: Index) {
        if let Some(idx) = self.map(index) {
            self.vector[idx] = T::default();
        }
    }
    /// Flip the vector
    pub fn reverse(&mut self) {
        self.vector.reverse();
    }
    /// Get item`T` on `index` as `Option<&T>`
    pub fn get(&self, index: Index) -> Option<&T> {
        if let Some(idx) = self.map(index) {
            self.vector.get(idx)
        } else {
            None
        }
    }
    /// Get clone of raw vector`Vec<T>`
    pub fn get_vec(&self) -> Vec<T> {
        self.vector.clone()
    }
    /// Get mutable item`T` on `index` as `Option<&mut T>`
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        if let Some(idx) = self.map(index) {
            self.vector.get_mut(idx)
        } else {
            None
        }
    }
    /// Is vector empty?
    pub fn is_empty(&self) -> bool {
        self.vector.is_empty()
    }
    /// Return size of `vector`
    pub fn size(&self) -> usize {
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
    /// Print a formatted vector
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
        vec.push(5, Index { x: 1, y: 1 }); // 5
        vec.fush(2, Index { x: 1, y: 1 }); // 2
        if let Some(value) = vec.get(Index { x: 1, y: 1 }) {
            println!("Value: {}", value);
        } else {
            println!("No value at this index");
        }

        println!("Size: {}", vec.size());
        vec.print();
    }
}
