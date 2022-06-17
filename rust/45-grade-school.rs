use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default)]
pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Self::default()
    }
    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .or_default()
            .insert(student.to_owned());
    }
    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .cloned()
            .collect()
    }
}
