#![allow(dead_code)]

// 390. Elimination Game
// https://leetcode.com/problems/elimination-game/
// https://leetcode.cn/problems/elimination-game/
//
// You have a list arr of all integers in the range [1, n] sorted in a strictly increasing order. Apply the following algorithm on arr:
//
// - Starting from left to right, remove the first number and every other number afterward until you reach the end of the list.
// - Repeat the previous step again, but this time from right to left, remove the rightmost number and every other number from the remaining numbers.
// - Keep repeating the steps again, alternating left to right and right to left, until a single number remains.
//
// Given the integer n, return the last number that remains in arr.
//
// Example 1:
//
// Input: n = 9
// Output: 6
// Explanation:
// arr = [1, 2, 3, 4, 5, 6, 7, 8, 9]
// arr = [2, 4, 6, 8]
// arr = [2, 6]
// arr = [6]
//
// Example 2:
//
// Input: n = 1
// Output: 1
//
// Constraints:
//
// - 1 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut n = n;
        let mut head = 1;
        let mut step = 1;
        let mut left = true;
        while n > 1 {
            if left || n % 2 == 1 {
                head += step;
            }
            n /= 2;
            step *= 2;
            left = !left;
        }
        head
    }
}

#[test]
fn test() {
    assert_eq!(Solution::last_remaining(9), 6);
    assert_eq!(Solution::last_remaining(1), 1);
}
