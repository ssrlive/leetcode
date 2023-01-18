#![allow(dead_code)]

// 1207. Unique Number of Occurrences
// https://leetcode.com/problems/unique-number-of-occurrences/
// https://leetcode.cn/problems/unique-number-of-occurrences/
//
// Easy
//
// Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.
//
// Example 1:
//
// Input: arr = [1,2,2,1,1,3]
// Output: true
// Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.
//
// Example 2:
//
// Input: arr = [1,2]
// Output: false
//
// Example 3:
//
// Input: arr = [-3,0,1,-3,1,1,1,-3,10,0]
// Output: true
//
// Constraints:
//
// -    1 <= arr.length <= 1000
// -    -1000 <= arr[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        for i in arr {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut set = std::collections::HashSet::new();
        for (_, v) in map {
            if set.contains(&v) {
                return false;
            }
            set.insert(v);
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 2, 1, 1, 3], true),
        (vec![1, 2], false),
        (vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0], true),
    ];
    for (arr, expected) in cases {
        assert_eq!(Solution::unique_occurrences(arr), expected);
    }
}
