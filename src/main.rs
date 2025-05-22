use crate::cache::{CacheEntry, CacheTable, Permission};

mod messages;
mod cache;

fn main() {
    println!("Hello, world!");
    
    let mut table = CacheTable::new(6);

    let entry1 = CacheEntry {
        address: 1,
        data: "data1".parse().unwrap(),
        dirty: false,
        permission: Permission::Shared,
    };
    table.insert(entry1.clone());
    
    let entry2 = CacheEntry {
        address: 2,
        data: "data2".parse().unwrap(),
        dirty: false,
        permission: Permission::Exclusive,
    };
    table.insert(entry2.clone());

    println!("Cache size: {}", table.len());

    for i in 0..(table.len() as i32) {
        println!("Entry {:?}: {:?}", i, table.get(&i));
    }
    
}
