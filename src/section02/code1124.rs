#![allow(dead_code)]

// 1124. Longest Well-Performing Interval
// https://leetcode.com/problems/longest-well-performing-interval/
// https://leetcode.cn/problems/longest-well-performing-interval/
//
// We are given hours, a list of the number of hours worked per day for a given employee.
//
// A day is considered to be a tiring day if and only if the number of hours worked is (strictly) greater than 8.
//
// A well-performing interval is an interval of days for which the number of tiring days is strictly larger than the number of non-tiring days.
//
// Return the length of the longest well-performing interval.
//
// Example 1:
//
// Input: hours = [9,9,6,0,6,6,9]
// Output: 3
// Explanation: The longest well-performing interval is [9,9,6].
//
// Example 2:
//
// Input: hours = [6,6,6]
// Output: 0
//
// Constraints:
//
// - 1 <= hours.length <= 10^4
// - 0 <= hours[i] <= 16
//

struct Solution;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut score = 0;
        let mut seen = std::collections::HashMap::new();
        for (i, &hour) in hours.iter().enumerate() {
            score += if hour > 8 { 1 } else { -1 };
            if score > 0 {
                res = i + 1;
            } else {
                seen.entry(score).or_insert(i);
                if seen.contains_key(&(score - 1)) {
                    res = res.max(i - seen[&(score - 1)]);
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let hours = vec![9, 9, 6, 0, 6, 6, 9];
    assert_eq!(Solution::longest_wpi(hours), 3);

    let hours = vec![6, 6, 6];
    assert_eq!(Solution::longest_wpi(hours), 0);
}
