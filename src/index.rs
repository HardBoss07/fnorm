use std::collections::HashMap;
use std::path::Path;

pub struct Indexer {
    global_mode: bool,
    start_index: usize,
    global_index: usize,
    per_dir: HashMap<String, usize>,
}

impl Indexer {
    pub fn new(global_mode: bool, start_index: usize) -> Self {
        Self {
            global_mode,
            // We subtract 1 because we increment *before* returning the value.
            // This makes the first index returned the actual start_index.
            start_index: start_index.saturating_sub(1),
            global_index: start_index.saturating_sub(1),
            per_dir: HashMap::new(),
        }
    }

    pub fn next(&mut self, parent: &Path) -> usize {
        if self.global_mode {
            self.global_index += 1;
            self.global_index
        } else {
            let key = parent.to_string_lossy().to_string();
            let entry = self.per_dir.entry(key).or_insert(self.start_index);
            *entry += 1;
            *entry
        }
    }
}
