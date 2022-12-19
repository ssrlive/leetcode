#![allow(dead_code)]

// 705. Design HashSet
// https://leetcode.com/problems/design-hashset/
// https://leetcode.cn/problems/design-hashset/
//
// Design a HashSet without using any built-in hash table libraries.
//
// Implement MyHashSet class:
//
// void add(key) Inserts the value key into the HashSet.
// bool contains(key) Returns whether the value key exists in the HashSet or not.
// void remove(key) Removes the value key in the HashSet. If key does not exist in the HashSet, do nothing.
//
// Example 1:
//
// Input
// ["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
// [[], [1], [2], [1], [3], [2], [2], [2], [2]]
// Output
// [null, null, null, true, false, null, true, null, false]
//
// Explanation
// MyHashSet myHashSet = new MyHashSet();
// myHashSet.add(1);      // set = [1]
// myHashSet.add(2);      // set = [1, 2]
// myHashSet.contains(1); // return True
// myHashSet.contains(3); // return False, (not found)
// myHashSet.add(2);      // set = [1, 2]
// myHashSet.contains(2); // return True
// myHashSet.remove(2);   // set = [1]
// myHashSet.contains(2); // return False, (already removed)
//
// Constraints:
//
// - 0 <= key <= 10^6
// - At most 10^4 calls will be made to add, remove, and contains.
//

struct MyHashSet {
    data: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            data: vec![false; 1000001],
        }
    }

    fn add(&mut self, key: i32) {
        self.data[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.data[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.data[key as usize]
    }
}

#[test]
fn test() {
    let mut my_hash_set = MyHashSet::new();
    my_hash_set.add(1);
    my_hash_set.add(2);
    assert_eq!(my_hash_set.contains(1), true);
    assert_eq!(my_hash_set.contains(3), false);
    my_hash_set.add(2);
    assert_eq!(my_hash_set.contains(2), true);
    my_hash_set.remove(2);
    assert_eq!(my_hash_set.contains(2), false);
}
