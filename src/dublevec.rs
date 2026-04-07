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
        Self {
            vector: vec![],
            count: vec![],
        }
    }
    fn offset(&self, index: Vec2) -> Option<usize> {
        if index.y >= self.count.len() {
            return None;
        }

        let row_len = self.count[index.y];
        if index.x >= row_len {
            return None;
        }

        Some(self.count.iter().take(index.y).sum::<usize>() + index.x)
    }
    pub fn as_slice(&self) -> &[T] {
        &self.vector
    }
    pub fn push(&mut self, v: Vec<T>) -> Vec<T> {
        self.vector.extend(v.clone());
        self.count.push(v.len());
        self.vector.clone()
    }

    pub fn remove(&mut self, index: Vec2) -> Option<Vec<T>> {
        if let Some(idx) = self.offset(index.clone()) {
            self.vector.remove(idx);
            if let Some(row_len) = self.count.get_mut(index.y) {
                *row_len -= 1;
                if *row_len == 0 {
                    self.count.remove(index.y);
                }
            }
            Some(self.vector.clone())
        } else {
            None
        }
    }

    pub fn pop_last(&mut self) -> Option<Vec<T>> {
        self.vector.pop();
        if let Some(v) = self.count.last_mut() {
            if *v == 0 {
                return None;
            } else {
                *v -= 1;
            }
        }
        Some(self.vector.clone())
    }
    pub fn pop_vec(&mut self) -> Option<Vec<T>> {
        if let Some(v) = self.count.last_mut() {
            for _ in 0..*v {
                self.vector.pop();
            }
        }
        self.count.pop();

        Some(self.vector.clone())
    }
    pub fn access(&self, index: Vec2) -> Option<&T> {
        if let Some(ofst) = self.offset(index) {
            Some(&self.vector[ofst])
        } else {
            None
        }
    }
    pub fn access_mut(&mut self, index: Vec2) -> Option<&T> {
        if let Some(ofst) = self.offset(index) {
            Some(&mut self.vector[ofst])
        } else {
            None
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

impl<T: fmt::Display> fmt::Display for DubleVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut start = 0;
        for &row_len in &self.count {
            for i in 0..row_len {
                write!(f, "{}", self.vector[start + i])?;
                if i != row_len - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f)?;
            start += row_len;
        }
        Ok(())
    }
}
