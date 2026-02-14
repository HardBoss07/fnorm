use std::collections::HashMap;
use std::path::Path;

pub struct Indexer {
    global_mode: bool,
    global_index: usize,
    per_dir: HashMap<String, usize>,
}

impl Indexer {
    pub fn new(global_mode: bool) -> Self {
        Self {
            global_mode,
            global_index: 0,
            per_dir: HashMap::new(),
        }
    }

    pub fn next(&mut self, parent: &Path) -> usize {
        if self.global_mode {
            self.global_index += 1;
            self.global_index
        } else {
            let key = parent.to_string_lossy().to_string();
            let entry = self.per_dir.entry(key).or_insert(0);
            *entry += 1;
            *entry
        }
    }
}
