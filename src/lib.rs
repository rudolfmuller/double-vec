/// The `Index` is a structure composed of variables of type `usize` named `x` and `y`
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
/// Fixed 2D vector
#[derive(Debug, Default, Clone)]
pub struct DubleVec<T> {
    scale: Size,
    vector: Vec<T>,
}
impl<T: Clone + PartialEq> DubleVec<T> {
    /// Creates a new `DubleVec`
    pub fn new(scale: Size, fill: T) -> Self {
        let vector = vec![fill.clone(); scale.w * scale.h];
        Self { scale, vector }
    }
    /// Get clone of raw vector`Vec<T>`
    pub fn access_vec(&self) -> Vec<T> {
        self.vector.clone()
    }
    pub fn as_slice(&self) -> &[T] {
        &self.vector
    }
    /// Get mutable item`T` on `index` as `Option<&mut T>`
    pub fn access_mut(&mut self, index: Index) -> Option<&mut T> {
        if index.x < self.scale.w && index.y < self.scale.h {
            Some(&mut self.vector[index.y * self.scale.w + index.x])
        } else {
            None
        }
    }
    /// Get item`T` on `index` as `Option<&T>`
    pub fn access(&self, index: Index) -> Option<&T> {
        if index.x < self.scale.w && index.y < self.scale.h {
            Some(&self.vector[index.y * self.scale.w + index.x])
        } else {
            None
        }
    }
    /// Set item`T` on `index`
    pub fn assign(&mut self, item: T, index: Index) {
        if let Some(idx) = self.access_mut(index) {
            *idx = item;
        }
    }
    /// Flip the vector
    pub fn reverse(&mut self) {
        self.vector.reverse();
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
                print!("{}", self.vector[idx]);
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
        let mut vec: DubleVec<i32> = DubleVec::new(Size { w: 6, h: 5 }, 0);
        vec.assign(5, Index { x: 1, y: 1 }); // 5
        if let Some(value) = vec.access(Index { x: 1, y: 1 }) {
            println!("Value: {}", value);
        } else {
            println!("No value at this index");
        }

        println!("Size: {}", vec.size());
        vec.print();
    }
}
