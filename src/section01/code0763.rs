#![allow(dead_code)]

// 763. Partition Labels
// https://leetcode.com/problems/partition-labels/
// https://leetcode.cn/problems/partition-labels/
//
// You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.
//
// Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
//
// Return a list of integers representing the size of these parts.
//
// Example 1:
//
// Input: s = "ababcbacadefegdehijhklij"
// Output: [9,7,8]
// Explanation:
// The partition is "ababcbaca", "defegde", "hijhklij".
// This is a partition so that each letter appears in at most one part.
// A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
//
// Example 2:
//
// Input: s = "eccbbbbdec"
// Output: [10]
//
// Constraints:
//
// - 1 <= s.length <= 500
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];
        for (i, c) in s.chars().enumerate() {
            last[(c as u8 - b'a') as usize] = i;
        }

        let mut res = vec![];
        let mut start = 0;
        let mut end = 0;
        for (i, c) in s.chars().enumerate() {
            end = end.max(last[(c as u8 - b'a') as usize]);
            if i == end {
                res.push((end - start + 1) as i32);
                start = end + 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "ababcbacadefegdehijhklij".to_string();
    let res = vec![9, 7, 8];
    assert_eq!(Solution::partition_labels(s), res);

    let s = "eccbbbbdec".to_string();
    let res = vec![10];
    assert_eq!(Solution::partition_labels(s), res);
}
