use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub struct SSTTableManager {
    memtable: BTreeMap<String, u64>,
    num_memtable_entries: u64,
    sst_table_filenames: Vec<String>,
}

impl SSTTableManager {
    pub fn new() -> SSTTableManager {
        SSTTableManager {
            memtable: BTreeMap::new(),
            num_memtable_entries: 0,
            sst_table_filenames: Vec::new(),
        }
    }

    fn flush_to_disk(&self) -> String {
        let filename = "/Users/susantu/rust_learning/db_engine/files/1.txt";
        let mut file = File::create(filename).unwrap();
        for (k,v) in &self.memtable {
            writeln!(&mut file, "{}", k);
            writeln!(&mut file, "{}", v);
        }        
        filename.to_string()
    }

    pub fn add(&mut self, key: &str, value: u64) {
        self.num_memtable_entries += 1;
        if self.num_memtable_entries > 3 {
            let filename = self.flush_to_disk();
            self.sst_table_filenames.insert(0, filename);
            self.memtable = BTreeMap::new();
            self.num_memtable_entries = 0;

        } else {
            self.memtable.insert(key.to_string(), value);
        }       
    }
}

fn main() {
    let mut manager = SSTTableManager::new();
    manager.add("a", 1234);
    manager.add("b", 1234);
    manager.add("c", 1234);
    manager.add("d", 1234);
}