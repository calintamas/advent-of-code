use std::collections::HashMap;
use std::hash::Hash;

pub struct UnionFind<T> {
    parent: Vec<usize>,
    size: Vec<usize>,
    val_to_idx: HashMap<T, usize>,
    idx_to_val: Vec<T>,
}

impl<T: Clone + Eq + Hash + Ord> UnionFind<T> {
    pub fn new() -> Self {
        UnionFind {
            parent: Vec::new(),
            size: Vec::new(),
            val_to_idx: HashMap::new(),
            idx_to_val: Vec::new(),
        }
    }

    pub fn make_set(&mut self, val: T) {
        if !self.val_to_idx.contains_key(&val) {
            let idx = self.parent.len();
            self.parent.push(idx);
            self.size.push(1);
            self.idx_to_val.push(val.clone());
            self.val_to_idx.insert(val, idx);
        }
    }

    pub fn find_set(&mut self, val: &T) -> Option<T> {
        let idx = *self.val_to_idx.get(val).unwrap();
        let root_idx = self.find_set_idx(idx);
        Some(self.idx_to_val[root_idx].clone())
    }

    pub fn find_set_idx(&mut self, idx: usize) -> usize {
        if idx == self.parent[idx] {
            return idx;
        }
        self.find_set_idx(self.parent[idx])
    }

    pub fn union_sets(&mut self, a: &T, b: &T) {
        if let (Some(&idx_a), Some(&idx_b)) = (self.val_to_idx.get(a), self.val_to_idx.get(b)) {
            let root_a = self.find_set_idx(idx_a);
            let root_b = self.find_set_idx(idx_b);
            if root_a != root_b {
                self.parent[root_b] = root_a;
                self.size[root_a] += self.size[root_b]; // update tree size
            }
        }
    }

    pub fn get_set_sizes(&self) -> Vec<usize> {
        self.parent
            .iter()
            .enumerate()
            .filter(|(i, &parent)| *i == parent) // only roots
            .map(|(i, _)| self.size[i])
            .collect()
    }
}
