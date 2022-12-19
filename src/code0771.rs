#![allow(dead_code)]

// 771. Jewels and Stones
// https://leetcode.com/problems/jewels-and-stones/
// https://leetcode.cn/problems/jewels-and-stones/
//
// You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have.
// Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.
//
// Letters are case sensitive, so "a" is considered a different type of stone from "A".
//
// Example 1:
//
// Input: jewels = "aA", stones = "aAAbbbb"
// Output: 3
//
// Example 2:
//
// Input: jewels = "z", stones = "ZZ"
// Output: 0
//
// Constraints:
//
// - 1 <= jewels.length, stones.length <= 50
// - jewels and stones consist of only English letters.
// - All the characters of jewels are unique.
//

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut count = 0;
        for c in stones.chars() {
            if jewels.contains(c) {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    let stones = "aAAbbbb".to_string();
    assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), stones), 3);
    assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}
