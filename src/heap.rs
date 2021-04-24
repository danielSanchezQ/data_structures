use std::cmp::Ordering;

struct Heap<T: PartialOrd + Clone> {
    nodes: Vec<T>,
    cmp_f: Box<fn(&T, &T) -> bool>,
}

impl<T: PartialOrd + Clone> Heap<T> {
    pub fn max() -> Self {
        Self {
            nodes: Vec::new(),
            cmp_f: Box::new(std::cmp::PartialOrd::gt),
        }
    }

    pub fn min() -> Self {
        Self {
            nodes: Vec::new(),
            cmp_f: Box::new(std::cmp::PartialOrd::lt),
        }
    }

    pub fn from_slice(slice: &[T], max: bool) -> Self {
        let cmp_f = Box::new(if max {
            std::cmp::PartialOrd::gt
        } else {
            std::cmp::PartialOrd::lt
        });
        let mut ret = Self {
            nodes: slice.to_vec(),
            cmp_f,
        };
        for i in 0..slice.len() / 2 {
            ret.swift_down(i);
        }
        ret
    }

    pub fn parent_index(i: usize) -> usize {
        i / 2
    }

    pub fn left_index(i: usize) -> usize {
        2 * i
    }

    pub fn right_index(i: usize) -> usize {
        2 * i + 1
    }

    pub fn get(&self, i: usize) -> &T {
        &self.nodes[i]
    }

    pub fn swift_up(&mut self, i: usize) {
        let cmp_f = self.cmp_f.as_ref();
        let mut parent = Self::parent_index(i);
        while i > 0 && cmp_f(self.get(i), self.get(i)) {
            self.nodes.swap(parent, i);
        }
    }

    pub fn swift_down(&mut self, mut i: usize) {
        let cmp_f = self.cmp_f.as_ref();

        loop {
            let mut max_index = i;

            let left_index = Self::left_index(i);
            if !cmp_f(self.get(left_index), self.get(max_index)) {
                max_index = left_index;
            }

            let right_index = Self::right_index(i);
            if !cmp_f(self.get(left_index), self.get(max_index)) {
                max_index = right_index;
            }

            if i == max_index {
                break;
            } else {
                self.nodes.swap(i, max_index);
                i = max_index;
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        self.nodes.push(value);
        self.swift_up(self.nodes.len() - 1);
    }

    pub fn extract_root(&mut self) -> T {
        let len = self.nodes.len() - 1;
        self.nodes.swap(0, len);
        let value = self.nodes.pop().unwrap();
        self.swift_down(0);
        value
    }

    pub fn insert_at(&mut self, i: usize, value: T) {
        let cmp_f = self.cmp_f.as_ref();

        let old_value = self.nodes.get_mut(i).unwrap();
        let swifting_up_or_down = cmp_f(old_value, &value);
        *old_value = value;
        if swifting_up_or_down {
            self.swift_up(i);
        } else {
            self.swift_down(i);
        }
    }

    pub fn remove(&mut self, i: usize) {
        *self.nodes.get_mut(i).unwrap() = self.nodes[0].clone();
        self.swift_up(i);
        self.extract_root();
    }
}
