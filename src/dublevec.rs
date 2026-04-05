use crate::vec2::Vec2;
use std::fmt;

/// Fixed 2D vector
#[derive(Debug, Default, Clone)]
pub struct DubleVec<T> {
    scale: Vec2,
    vector: Vec<T>,
}
impl<T: Clone + PartialEq> DubleVec<T> {
    /// Creates a new `DubleVec`
    pub fn new(scale: Vec2, fill: T) -> Self {
        let vector = vec![fill.clone(); scale.x * scale.y];
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
    pub fn access_mut(&mut self, index: Vec2) -> Option<&mut T> {
        if index.x < self.scale.x && index.y < self.scale.y {
            Some(&mut self.vector[index.y * self.scale.x + index.x])
        } else {
            None
        }
    }
    /// Get item`T` on `index` as `Option<&T>`
    pub fn access(&self, index: Vec2) -> Option<&T> {
        if index.x < self.scale.x && index.y < self.scale.y {
            Some(&self.vector[index.y * self.scale.x + index.x])
        } else {
            None
        }
    }
    /// Set item`T` on `index`
    pub fn assign(&mut self, item: T, index: Vec2) {
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
impl<T> From<Vec<Vec<T>>> for DubleVec<T> {
    /// Converts `Vec<Vec<T>>` to `DubleVec<T>`

    /// MUST BE A RECTANGULAR TYPE!!!
    fn from(other: Vec<Vec<T>>) -> Self {
        let h = other.len();
        let w = other.first().map(|row| row.len()).unwrap_or(0);

        let vector = other.into_iter().flatten().collect();

        let scale = Vec2 { x: w, y: h };

        Self { scale, vector }
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

impl<T: fmt::Display> fmt::Display for DubleVec<T> {
    /// Print a formatted vector
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.scale.y {
            for x in 0..self.scale.x {
                let idx = y * self.scale.x + x;
                write!(f, "{}, ", self.vector[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
