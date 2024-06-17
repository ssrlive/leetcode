#![allow(dead_code)]

// 3185. Count Pairs That Form a Complete Day II
// https://leetcode.com/problems/count-pairs-that-form-a-complete-day-ii/
// https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-ii/
//
// Medium
//
// Given an integer array hours representing times in hours, return an integer denoting the
// number of pairs i, j where i < j and hours[i] + hours[j] forms a complete day.
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
// Explanation: The pairs of indices that form a complete day are (0, 1) and (3, 4).
//
// Example 2:
//
// Input: hours = [72,48,24,3]
//
// Output: 3
//
// Explanation: The pairs of indices that form a complete day are (0, 1), (0, 2), and (1, 2).
//
// Constraints:
//
//     1 <= hours.length <= 5 * 10^5
//     1 <= hours[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut h = std::collections::HashMap::new();

        for mut t in hours {
            t %= 24;

            *h.entry(t).or_insert(0) += 1;
        }

        let mut r = 0;
        let mut s = 0;

        for (&t, &_c) in &h {
            let p = (24 - t) % 24;

            if h.contains_key(&p) {
                if p == t {
                    s += (h[&t] * (h[&t] - 1)) / 2;
                } else {
                    r += h[&t] * h[&p];
                }
            }
        }

        s + r / 2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]), 2);
    assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
}
