#![allow(dead_code)]

// 432. All O`one Data Structure
// https://leetcode.com/problems/all-oone-data-structure/
// https://leetcode.cn/problems/all-oone-data-structure/
//
// Design a data structure to store the strings' count with the ability to return the strings with minimum and maximum counts.
//
// Implement the AllOne class:
//
// - AllOne() Initializes the object of the data structure.
// - inc(String key) Increments the count of the string key by 1. If key does not exist in the data structure, insert it with count 1.
// - dec(String key) Decrements the count of the string key by 1. If the count of key is 0 after the decrement, remove it from the data structure. It is guaranteed that key exists in the data structure before the decrement.
// - getMaxKey() Returns one of the keys with the maximal count. If no element exists, return an empty string "".
// - getMinKey() Returns one of the keys with the minimum count. If no element exists, return an empty string "".
//
// Note that each function must run in O(1) average time complexity.
//
// Example 1:
//
// Input
// ["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
// [[], ["hello"], ["hello"], [], [], ["leet"], [], []]
// Output
// [null, null, null, "hello", "hello", null, "hello", "leet"]
//
// Explanation
// AllOne allOne = new AllOne();
// allOne.inc("hello");
// allOne.inc("hello");
// allOne.getMaxKey(); // return "hello"
// allOne.getMinKey(); // return "hello"
// allOne.inc("leet");
// allOne.getMaxKey(); // return "hello"
// allOne.getMinKey(); // return "leet"
//
// Constraints:
//
// - 1 <= key.length <= 10
// - key consists of lowercase English letters.
// - It is guaranteed that for each call to dec, key is existing in the data structure.
// - At most 5 * 104 calls will be made to inc, dec, getMaxKey, and getMinKey.
//

// use std::collections::HashMap;
struct AllOne {
    one: std::collections::HashMap<String, i32>,
}
impl AllOne {
    fn new() -> Self {
        let m = std::collections::HashMap::new();
        AllOne { one: m }
    }
    fn inc(&mut self, key: String) {
        if let Some(v) = self.one.get_mut(&key) {
            *v += 1;
        } else {
            self.one.insert(key, 1);
        }
    }
    fn dec(&mut self, key: String) {
        if let Some(v) = self.one.get_mut(&key) {
            *v -= 1;
            if *v < 1 {
                self.one.remove(&key);
            }
        }
    }
    fn get_max_key(&self) -> String {
        let mut max = 0;
        let mut s_of_max = String::new();
        for (i, v) in &self.one {
            if *v > max {
                s_of_max = i.to_string();
                max = *v;
            }
        }
        s_of_max
    }
    fn get_min_key(&self) -> String {
        let mut min = 2147483647;
        let mut s_of_min = String::new();
        for (i, v) in &self.one {
            if *v < min {
                s_of_min = i.to_string();
                min = *v;
            }
        }
        s_of_min
    }
}

#[test]
fn test_all_one() {
    let mut all_one = AllOne::new();
    all_one.inc("hello".to_string());
    all_one.inc("hello".to_string());
    assert_eq!(all_one.get_max_key(), "hello".to_string());
    assert_eq!(all_one.get_min_key(), "hello".to_string());
    all_one.inc("leet".to_string());
    assert_eq!(all_one.get_max_key(), "hello".to_string());
    assert_eq!(all_one.get_min_key(), "leet".to_string());
}
