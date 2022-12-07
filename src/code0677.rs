#![allow(dead_code)]

// 677. Map Sum Pairs
// https://leetcode.com/problems/map-sum-pairs/
//
// Design a map that allows you to do the following:
//
// - Maps a string key to a given value.
// - Returns the sum of the values that have a key with a prefix equal to a given string.
//
// Implement the MapSum class:
//
// - MapSum() Initializes the MapSum object.
// - void insert(String key, int val) Inserts the key-val pair into the map. If the key already existed,
//   the original key-value pair will be overridden to the new one.
// - int sum(string prefix) Returns the sum of all the pairs' value whose key starts with the prefix.
//
// Example 1:
//
// Input
// ["MapSum", "insert", "sum", "insert", "sum"]
// [[], ["apple", 3], ["ap"], ["app", 2], ["ap"]]
// Output
// [null, null, 3, null, 5]
//
// Explanation
// MapSum mapSum = new MapSum();
// mapSum.insert("apple", 3);
// mapSum.sum("ap");           // return 3 (apple = 3)
// mapSum.insert("app", 2);
// mapSum.sum("ap");           // return 5 (apple + app = 3 + 2 = 5)
//
// Constraints:
//
// - 1 <= key.length, prefix.length <= 50
// - key and prefix consist of only lowercase English letters.
// - 1 <= val <= 1000
// - At most 50 calls will be made to insert and sum.
//

use std::collections::HashMap;

struct MapSum {
    map: HashMap<String, i32>,
}

impl MapSum {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.map.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.map
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .sum()
    }
}

#[test]
fn test() {
    let mut map_sum = MapSum::new();
    map_sum.insert("apple".to_string(), 3);
    assert_eq!(map_sum.sum("ap".to_string()), 3);
    map_sum.insert("app".to_string(), 2);
    assert_eq!(map_sum.sum("ap".to_string()), 5);
}
