#![allow(dead_code)]

/*

// 2269. Find the K-Beauty of a Number
// https://leetcode.com/problems/find-the-k-beauty-of-a-number/
// https://leetcode.cn/problems/find-the-k-beauty-of-a-number/
//
// Easy
//
// The k-beauty of an integer num is defined as the number of substrings of num when it is read as a string that meet the following conditions:

    It has a length of k.
    It is a divisor of num.

Given integers num and k, return the k-beauty of num.

Note:

    Leading zeros are allowed.
    0 is not a divisor of any value.

A substring is a contiguous sequence of characters in a string.

Example 1:

Input: num = 240, k = 2
Output: 2
Explanation: The following are the substrings of num of length k:
- "24" from "240": 24 is a divisor of 240.
- "40" from "240": 40 is a divisor of 240.
Therefore, the k-beauty is 2.

Example 2:

Input: num = 430043, k = 2
Output: 2
Explanation: The following are the substrings of num of length k:
- "43" from "430043": 43 is a divisor of 430043.
- "30" from "430043": 30 is not a divisor of 430043.
- "00" from "430043": 0 is not a divisor of 430043.
- "04" from "430043": 4 is not a divisor of 430043.
- "43" from "430043": 43 is a divisor of 430043.
Therefore, the k-beauty is 2.

Constraints:

    1 <= num <= 10^9
    1 <= k <= num.length (taking num as a string)
*/

struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut value = String::new();
        let mut counter = 0;
        for ch in num.to_string().chars() {
            if value.len() != k as usize {
                value.push(ch);
                continue;
            }
            let digit = value.parse::<i32>().unwrap();
            if digit > 0 && num % digit == 0 {
                counter += 1;
            }
            value.remove(0);
            value.push(ch);
        }
        if !value.is_empty() {
            let digit = value.parse::<i32>().unwrap();
            if digit > 0 && num % digit == 0 {
                counter += 1;
            }
        }
        counter
    }
}

#[test]
fn test() {
    assert_eq!(Solution::divisor_substrings(240, 2), 2);
    assert_eq!(Solution::divisor_substrings(430043, 2), 2);
    assert_eq!(Solution::divisor_substrings(430043, 3), 1);
}
