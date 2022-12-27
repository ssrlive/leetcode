#![allow(dead_code)]

// 2405. Optimal Partition of String
// https://leetcode.com/problems/optimal-partition-of-string/
// https://leetcode.cn/problems/optimal-partition-of-string/
//
// Given a string s, partition the string into one or more substrings such that the characters in each substring are unique.
// That is, no letter appears in a single substring more than once.
//
// Return the minimum number of substrings in such a partition.
//
// Note that each character should belong to exactly one substring in a partition.
//
// Example 1:
//
// Input: s = "abacaba"
// Output: 4
// Explanation:
// Two possible partitions are ("a","ba","cab","a") and ("ab","a","ca","ba").
// It can be shown that 4 is the minimum number of substrings needed.
//
// Example 2:
//
// Input: s = "ssssss"
// Output: 6
// Explanation:
// The only valid partition is ("s","s","s","s","s","s").
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of only English lowercase letters.
//

struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        use std::collections::HashSet;
        let mut ans = 1;
        let mut cs = HashSet::new();
        for c in s.chars() {
            if cs.contains(&c) {
                ans += 1;
                cs.clear();
            }
            cs.insert(c);
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::partition_string("abacaba".to_string()), 4);
    assert_eq!(Solution::partition_string("ssssss".to_string()), 6);
}
