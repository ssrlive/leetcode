#![allow(dead_code)]

// 3184. Count Pairs That Form a Complete Day I
// https://leetcode.com/problems/count-pairs-that-form-a-complete-day-i/
// https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-i/
//
// Easy
//
// Given an integer array hours representing times in hours, return an integer denoting
// the number of pairs i, j where i < j and hours[i] + hours[j] forms a complete day.
//
// A complete day is defined as a time duration that is an exact multiple of 24 hours.
//
// For example, 1 day is 24 hours, 2 days is 48 hours, 3 days is 72 hours, and so on.
//
// Example 1:
//
// Input: hours = [12,12,30,24,24]
//
// Output: 2
//
// Explanation:
//
// The pairs of indices that form a complete day are (0, 1) and (3, 4).
//
// Example 2:
//
// Input: hours = [72,48,24,3]
//
// Output: 3
//
// Explanation:
//
// The pairs of indices that form a complete day are (0, 1), (0, 2), and (1, 2).
//
// Constraints:
//
//     1 <= hours.length <= 100
//     1 <= hours[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut ump = std::collections::HashMap::new();
        for h in hours {
            ans += *ump.entry((24 - h % 24) % 24).or_insert(0);
            *ump.entry(h % 24).or_insert(0) += 1;
        }
        ans
    }
}

#[test]
fn test() {
    let hours = vec![12, 12, 30, 24, 24];
    let res = 2;
    assert_eq!(Solution::count_complete_day_pairs(hours), res);

    let hours = vec![72, 48, 24, 3];
    let res = 3;
    assert_eq!(Solution::count_complete_day_pairs(hours), res);
}
