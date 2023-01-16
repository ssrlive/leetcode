#![allow(dead_code)]

// 2522. Partition String Into Substrings With Values at Most K
// https://leetcode.com/problems/partition-string-into-substrings-with-values-at-most-k/
// https://leetcode.cn/problems/partition-string-into-substrings-with-values-at-most-k/
//
// You are given a string s consisting of digits from 1 to 9 and an integer k.
//
// A partition of a string s is called good if:
//
// Each digit of s is part of exactly one substring.
// The value of each substring is less than or equal to k.
// Return the minimum number of substrings in a good partition of s. If no good partition of s exists, return -1.
//
// Note that:
//
// The value of a string is its result when interpreted as an integer. For example, the value of "123" is 123 and the value of "1" is 1.
// A substring is a contiguous sequence of characters within a string.
//
// Example 1:
//
// Input: s = "165462", k = 60
// Output: 4
// Explanation: We can partition the string into substrings "16", "54", "6", and "2". Each substring has a value less than or equal to k = 60.
// It can be shown that we cannot partition the string into less than 4 substrings.
//
// Example 2:
//
// Input: s = "238182", k = 5
// Output: -1
// Explanation: There is no good partition for this string.
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s[i] is a digit from '1' to '9'.
// - 1 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn minimum_partition(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut index = 0;
        let s = s.as_bytes();
        let k = k as i64;
        while index < s.len() as i32 {
            let mut sum = 0;
            let mut flag = false;
            while sum <= k && index < s.len() as i32 {
                if index >= 0 && flag {
                    index += 1;
                }
                sum *= 10;
                sum += *s.get(index as usize).unwrap_or(&0) as i64 - '0' as i64;
                if !flag && sum > k {
                    return -1;
                }
                flag = true;
            }
            index -= 1;
            if sum == (s[index as usize] - b'0') as i64 && sum > k {
                return -1;
            }
            ans += 1;
            index += 1;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_partition("165462".to_string(), 60), 4);
    assert_eq!(Solution::minimum_partition("238182".to_string(), 5), -1);
}
