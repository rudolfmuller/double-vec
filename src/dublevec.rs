use crate::vec2::Vec2;
use std::fmt::{self, Display};
// 1, 2, 3, 4,
// push([1,2,3])
// 1, 2, 3, 4, 1, 2, 3
/// Fixed 2D vector
#[derive(Debug, Default, Clone)]
pub struct DubleVec<T> {
    vector: Vec<T>,
    count: Vec<usize>,
}
impl<T: Clone + PartialEq + Display> DubleVec<T> {
    /// Creates a new `DubleVec`
    pub fn new() -> Self {
        let vector: Vec<T> = Vec::new();
        Self {
            vector,
            count: vec![],
        }
    }
    pub fn as_slice(&self) -> &[T] {
        &self.vector
    }
    pub fn push(&mut self, v: Vec<T>) {
        self.vector.extend(v.clone());
        self.count.push(v.len());
    }
    pub fn access(&self, index: Vec2) -> Option<&T> {
        if index.y >= self.count.len() {
            return None;
        }

        let row_len = self.count[index.y];
        if index.x >= row_len {
            return None;
        }

        let offset: usize = self.count.iter().take(index.y).sum();
        Some(&self.vector[offset + index.x])
    }
    pub fn access_mut(&mut self, index: Vec2) -> Option<&mut T> {
        if index.y >= self.count.len() {
            return None;
        }

        let row_len = self.count[index.y];
        if index.x >= row_len {
            return None;
        }

        let offset: usize = self.count.iter().take(index.y).sum();
        Some(&mut self.vector[offset + index.x])
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

    pub fn dbg(&self) {
        for c in self.vector.clone() {
            println!("{}", c);
        }
    }
}
