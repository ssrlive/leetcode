#![allow(dead_code)]

// 460. LFU Cache
// https://leetcode.com/problems/lfu-cache/
// https://leetcode.cn/problems/lfu-cache/
//
// Design and implement a data structure for Least Frequently Used (LFU) cache.
//
// Implement the LFUCache class:
//
// - LFUCache(int capacity) Initializes the object with the capacity of the data structure.
// - int get(int key) Gets the value of the key if the key exists in the cache. Otherwise, returns -1.
// - void put(int key, int value) Update the value of the key if present, or inserts the key if not already present. When the cache reaches its capacity, it should invalidate and remove the least frequently used key before inserting a new item. For this problem, when there is a tie (i.e., two or more keys with the same frequency), the least recently used key would be evicted.
//
// To determine the least frequently used key, a use counter is maintained for each key in the cache. The key with the smallest use counter is the least frequently used key.
//
// When a key is first inserted into the cache, its use counter is set to 1 (due to the put operation). The use counter for a key in the cache is incremented either a get or put operation is called on it.
//
// The functions get and put must each run in O(1) average time complexity.
//
// Example 1:
//
// Input
// ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
// [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
// Output
// [null, null, null, 1, null, -1, 3, null, -1, 3, 4]
//
// Explanation
// // cnt(x) = the use counter for key x
// // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
// LFUCache lfu = new LFUCache(2);
// lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
// lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
// lfu.get(1);      // return 1
//                  // cache=[1,2], cnt(2)=1, cnt(1)=2
// lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
//                  // cache=[3,1], cnt(3)=1, cnt(1)=2
// lfu.get(2);      // return -1 (not found)
// lfu.get(3);      // return 3
//                  // cache=[3,1], cnt(3)=2, cnt(1)=2
// lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
//                  // cache=[4,3], cnt(4)=1, cnt(3)=2
// lfu.get(1);      // return -1 (not found)
// lfu.get(3);      // return 3
//                  // cache=[3,4], cnt(4)=1, cnt(3)=3
// lfu.get(4);      // return 4
//                  // cache=[3,4], cnt(4)=2, cnt(3)=3
//
// Constraints:
//
// - 0 <= capacity, key, value <= 10^4
// - At most 10^5 calls will be made to get and put.
//

use std::collections::HashMap;

struct LFUCache {
    capacity: usize,
    min_freq: usize,
    freq: HashMap<usize, Vec<usize>>,
    key_to_val: HashMap<usize, usize>,
    key_to_freq: HashMap<usize, usize>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            min_freq: 0,
            freq: HashMap::new(),
            key_to_val: HashMap::new(),
            key_to_freq: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self._get(key).unwrap_or_default()
    }
    fn _get(&mut self, key: i32) -> Option<i32> {
        let key = key as usize;
        if !self.key_to_val.contains_key(&key) {
            return Some(-1);
        }

        let freq = self.key_to_freq.get(&key)?;
        self.freq.get_mut(freq)?.retain(|&k| k != key);
        if self.freq.get(freq)?.is_empty() {
            self.freq.remove(freq);
            if self.min_freq == *freq {
                self.min_freq += 1;
            }
        }

        let freq = freq + 1;
        self.freq.entry(freq).or_default().push(key);
        self.key_to_freq.insert(key, freq);

        Some(*self.key_to_val.get(&key)? as i32)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self._put(key, value).unwrap_or_default();
    }
    fn _put(&mut self, key: i32, value: i32) -> Option<()> {
        let key = key as usize;
        let value = value as usize;
        if self.capacity == 0 {
            return Some(());
        }

        if let std::collections::hash_map::Entry::Occupied(mut e) = self.key_to_val.entry(key) {
            e.insert(value);
            self.get(key as i32);
            return Some(());
        }

        if self.key_to_val.len() == self.capacity {
            let key = self.freq.get_mut(&self.min_freq)?.remove(0);
            self.key_to_val.remove(&key);
            self.key_to_freq.remove(&key);
            if self.freq.get(&self.min_freq)?.is_empty() {
                self.freq.remove(&self.min_freq);
            }
        }

        self.key_to_val.insert(key, value);
        self.key_to_freq.insert(key, 1);
        self.freq.entry(1).or_default().push(key);
        self.min_freq = 1;
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
