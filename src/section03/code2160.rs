#![allow(dead_code)]

/*

// 2160. Minimum Sum of Four Digit Number After Splitting Digits
// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
// https://leetcode.cn/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
//
// Easy
//
// You are given a positive integer num consisting of exactly four digits. Split num into two new integers new1 and new2 by using the digits found in num. Leading zeros are allowed in new1 and new2, and all the digits found in num must be used.

    For example, given num = 2932, you have the following digits: two 2's, one 9 and one 3. Some of the possible pairs [new1, new2] are [22, 93], [23, 92], [223, 9] and [2, 329].

Return the minimum possible sum of new1 and new2.

Example 1:

Input: num = 2932
Output: 52
Explanation: Some possible pairs [new1, new2] are [29, 23], [223, 9], etc.
The minimum sum can be obtained by the pair [29, 23]: 29 + 23 = 52.

Example 2:

Input: num = 4009
Output: 13
Explanation: Some possible pairs [new1, new2] are [0, 49], [490, 0], etc.
The minimum sum can be obtained by the pair [4, 9]: 4 + 9 = 13.

Constraints:

    1000 <= num <= 9999
*/

struct Solution;

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut num = num;
        let mut digits = Vec::with_capacity(4);
        (0..4).for_each(|_| {
            digits.push(num % 10);
            num /= 10;
        });
        digits.sort_unstable();
        digits
            .into_iter()
            .enumerate()
            .fold(0, |sum, (i, x)| sum + x * if i < 2 { 10 } else { 1 })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_sum(2932), 52);
    assert_eq!(Solution::minimum_sum(4009), 13);
}
