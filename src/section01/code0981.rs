#![allow(dead_code)]

// 981. Time Based Key-Value Store
// https://leetcode.com/problems/time-based-key-value-store/
// https://leetcode.cn/problems/time-based-key-value-store/
//
// Design a time-based key-value data structure that can store multiple values for the same key
// at different time stamps and retrieve the key's value at a certain timestamp.
//
// Implement the TimeMap class:
//
// TimeMap() Initializes the object of the data structure.
// void set(String key, String value, int timestamp) Stores the key key with the value value at the given time timestamp.
// String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp.
// If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no values, it returns "".
//
// Example 1:
//
// Input
// ["TimeMap", "set", "get", "get", "set", "get", "get"]
// [[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]]
// Output
// [null, null, "bar", "bar", null, "bar2", "bar2"]
//
// Explanation
// TimeMap timeMap = new TimeMap();
// timeMap.set("foo", "bar", 1);  // store the key "foo" and value "bar" along with timestamp = 1.
// timeMap.get("foo", 1);         // return "bar"
// timeMap.get("foo", 3);         // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
// timeMap.set("foo", "bar2", 4); // store the key "foo" and value "bar2" along with timestamp = 4.
// timeMap.get("foo", 4);         // return "bar2"
// timeMap.get("foo", 5);         // return "bar2"
//
// Constraints:
//
// - 1 <= key.length, value.length <= 100
// - key and value consist of lowercase English letters and digits.
// - 1 <= timestamp <= 107
// - All the timestamps timestamp of set are strictly increasing.
// - At most 2 * 105 calls will be made to set and get.
//

use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { map: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .and_then(|v| match v.partition_point(|(t, _)| *t <= timestamp) {
                0 => None,
                i => Some(v[i - 1].1.clone()),
            })
            .unwrap_or_default()
    }
}

#[test]
fn test() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
    time_map.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
}
