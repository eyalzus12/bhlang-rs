use std::collections::HashMap;

pub struct LangFile {
    pub header: u32,
    pub entries: HashMap<String, String>,
}

impl LangFile {
    pub fn new(header: u32) -> Self {
        Self {
            header,
            entries: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.entries.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.values()
    }
}
