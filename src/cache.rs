use std::collections::{HashMap, LinkedList};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Permission {
    Invalid,
    Shared,
    Exclusive,
    Modified
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CacheEntry {
    pub address: i32,
    pub data: String,
    pub dirty: bool,
    pub permission: Permission,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CacheTable {
    entries: HashMap<i32, CacheEntry>,
    // `order` stores the addresses (keys) in a LinkedList.
    // The front of the list will be the Most Recently Used (MRU) entry.
    // The back of the list will be the Least Recently Used (LRU) entry.
    order: LinkedList<i32>,
    capacity: usize,
}

impl CacheTable {
    pub fn new(capacity: usize) -> Self {
        CacheTable {
            entries: HashMap::with_capacity(capacity),
            order: LinkedList::new(),
            capacity,
        }
    }

    // Retrieves an entry, making it the Most Recently Used.
    pub fn get(&mut self, address: &i32) -> Option<&CacheEntry> {
        if self.entries.contains_key(address) {
            // If the entry exists, move its key to the front of the 'order' list.
            self.order.retain(|&x| x != *address); // Remove existing instance
            self.order.push_front(*address); // Add to front
            self.entries.get(address)
        } else {
            None
        }
    }

    // Inserts a new entry or updates an existing one,
    // handling eviction if the cache is full.
    pub fn insert(&mut self, entry: CacheEntry) {
        let address = entry.address;

        if self.entries.contains_key(&address) {
            // If the entry already exists, update its data and move to MRU.
            self.entries.insert(address, entry);
            self.order.retain(|&x| x != address);
            self.order.push_front(address);
            return;
        }

        // If the cache is at capacity, evict the LRU entry.
        if self.entries.len() >= self.capacity {
            if let Some(lru_address) = self.order.pop_back() {
                // Remove the LRU entry from the HashMap.
                self.entries.remove(&lru_address);
                println!("Evicted LRU entry with address: 0x{:x}", lru_address);
            }
        }

        // Insert the new entry into the HashMap and make it MRU.
        self.entries.insert(address, entry);
        self.order.push_front(address);
    }

    // Removes an entry from the cache and updates LRU order.
    pub fn remove(&mut self, address: &i32) -> Option<CacheEntry> {
        if let Some(entry) = self.entries.remove(address) {
            // Remove the key from the order list as well.
            self.order.retain(|&x| x != *address);
            Some(entry)
        } else {
            None
        }
    }

    // Updates the permission of an entry, making it Most Recently Used.
    pub fn update_permission(&mut self, address: i32, new_permission: Permission) -> bool {
        if let Some(entry) = self.entries.get_mut(&address) {
            entry.permission = new_permission;
            // Accessing/modifying also makes it MRU.
            self.order.retain(|&x| x != address);
            self.order.push_front(address);
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

