#![allow(dead_code)]

// 2437. Number of Valid Clock Times
// https://leetcode.com/problems/number-of-valid-clock-times/
// https://leetcode.cn/problems/number-of-valid-clock-times/
//
// You are given a string of length 5 called time, representing the current time on a digital clock in the format "hh:mm".
// The earliest possible time is "00:00" and the latest possible time is "23:59".
//
// In the string time, the digits represented by the ? symbol are unknown, and must be replaced with a digit from 0 to 9.
//
// Return an integer answer, the number of valid clock times that can be created by replacing every ? with a digit from 0 to 9.
//
// Example 1:
//
// Input: time = "?5:00"
// Output: 2
// Explanation: We can replace the ? with either a 0 or 1, producing "05:00" or "15:00". Note that we cannot replace it with a 2,
// since the time "25:00" is invalid. In total, we have two choices.
//
// Example 2:
//
// Input: time = "0?:0?"
// Output: 100
// Explanation: Each ? can be replaced by any digit from 0 to 9, so we have 100 total choices.
//
// Example 3:
//
// Input: time = "??:??"
// Output: 1440
// Explanation: There are 24 possible choices for the hours, and 60 possible choices for the minutes. In total, we have 24 * 60 = 1440 choices.
//
// Constraints:
//
// - time is a valid string of length 5 in the format "hh:mm".
// - "00" <= hh <= "23"
// - "00" <= mm <= "59"
// - Some of the digits might be replaced with '?' and need to be replaced with digits from 0 to 9.
//

struct Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut res = 1;
        if let &[h1, h2, _, m1, m2] = time.as_bytes() {
            res *= match (h1, h2) {
                (b'?', b'?') => 24,
                (b'?', b'0'..=b'3') => 3,
                (b'?', _) => 2,
                (b'2', b'?') => 4,
                (_, b'?') => 10,
                _ => 1,
            };
            res *= if m1 == b'?' { 6 } else { 1 };
            res *= if m2 == b'?' { 10 } else { 1 };
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_time("?5:00".to_string()), 2);
    assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
    assert_eq!(Solution::count_time("??:??".to_string()), 1440);
}
