use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
pub struct School(HashMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0.entry(grade).or_default().insert(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut vec: Vec<_> = self.0.keys().copied().collect();
        vec.sort();
        vec
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.0.get(&grade).map(|g| g.iter().cloned().collect())
    }
}