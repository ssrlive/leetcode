#![allow(dead_code)]

// 3114. Latest Time You Can Obtain After Replacing Characters
// https://leetcode.com/problems/latest-time-you-can-obtain-after-replacing-characters/
// https://leetcode.cn/problems/latest-time-you-can-obtain-after-replacing-characters/
//
// Easy
//
// You are given a string s representing a 12-hour format time where some of the digits (possibly none) are replaced with a "?".
//
// 12-hour times are formatted as "HH:MM", where HH is between 00 and 11, and MM is between 00 and 59. The earliest 12-hour time is 00:00, and the latest is 11:59.
//
// You have to replace all the "?" characters in s with digits such that the time we obtain by the resulting string is a valid 12-hour format time and is the latest possible.
//
// Return the resulting string.
//
// Example 1:
//
// Input: s = "1?:?4"
//
// Output: "11:54"
//
// Explanation: The latest 12-hour format time we can achieve by replacing "?" characters is "11:54".
//
// Example 2:
//
// Input: s = "0?:5?"
//
// Output: "09:59"
//
// Explanation: The latest 12-hour format time we can achieve by replacing "?" characters is "09:59".
//
// Constraints:
//
//     s.length == 5
//     s[2] is equal to the character ":".
//     All characters except s[2] are digits or "?" characters.
//     The input is generated such that there is at least one time between "00:00" and "11:59" that you can obtain after replacing the "?" characters.
//

struct Solution;

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut b = s.into_bytes();
        b[4] = if b[4] == b'?' { b'9' } else { b[4] };
        b[3] = if b[3] == b'?' { b'5' } else { b[3] };
        b[1] = if b[1] == b'?' {
            if b[0] == b'0' {
                b'9'
            } else {
                b'1'
            }
        } else {
            b[1]
        };
        b[0] = if b[0] == b'?' {
            if b[1] == b'0' || b[1] == b'1' {
                b'1'
            } else {
                b'0'
            }
        } else {
            b[0]
        };
        String::from_utf8(b).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_latest_time("1?:?4".to_string()), "11:54");
    assert_eq!(Solution::find_latest_time("0?:5?".to_string()), "09:59");
}
