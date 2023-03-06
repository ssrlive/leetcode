#![allow(dead_code)]

/*

// 2231. Largest Number After Digit Swaps by Parity
// https://leetcode.com/problems/largest-number-after-digit-swaps-by-parity/
// https://leetcode.cn/problems/largest-number-after-digit-swaps-by-parity/
//
// Easy
//
// You are given a positive integer num. You may swap any two digits of num that have the same parity (i.e. both odd digits or both even digits).

Return the largest possible value of num after any number of swaps.

Example 1:

Input: num = 1234
Output: 3412
Explanation: Swap the digit 3 with the digit 1, this results in the number 3214.
Swap the digit 2 with the digit 4, this results in the number 3412.
Note that there may be other sequences of swaps but it can be shown that 3412 is the largest possible number.
Also note that we may not swap the digit 4 with the digit 1 since they are of different parities.

Example 2:

Input: num = 65875
Output: 87655
Explanation: Swap the digit 8 with the digit 6, this results in the number 85675.
Swap the first digit 5 with the digit 7, this results in the number 87655.
Note that there may be other sequences of swaps but it can be shown that 87655 is the largest possible number.

Constraints:

    1 <= num <= 10^9
*/

struct Solution;

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut num = num as usize;
        let mut evens_odds = [BinaryHeap::new(), BinaryHeap::new()];
        let mut digits = Vec::new();
        while num > 0 {
            let dig = num % 10;
            evens_odds[dig % 2].push(dig as i32);
            digits.push(dig);
            num /= 10;
        }
        let f = |res: i32, &dig: &usize| 10 * res + evens_odds[dig % 2].pop().unwrap();
        digits.iter().rev().fold(0, f)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_integer(1234), 3412);
    assert_eq!(Solution::largest_integer(65875), 87655);
}
