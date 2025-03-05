#![allow(dead_code)]

/*

// 2400. Number of Ways to Reach a Position After Exactly k Steps
// https://leetcode.com/problems/number-of-ways-to-reach-a-position-after-exactly-k-steps/
// https://leetcode.cn/problems/number-of-ways-to-reach-a-position-after-exactly-k-steps/
//
// Medium
//
// You are given two positive integers startPos and endPos.
// Initially, you are standing at position startPos on an infinite number line.
// With one step, you can move either one position to the left, or one position to the right.

Given a positive integer k, return the number of different ways to reach the position endPos starting from startPos, such that you perform exactly k steps. Since the answer may be very large, return it modulo 109 + 7.

Two ways are considered different if the order of the steps made is not exactly the same.

Note that the number line includes negative integers.

Example 1:

Input: startPos = 1, endPos = 2, k = 3
Output: 3
Explanation: We can reach position 2 from 1 in exactly 3 steps in three ways:
- 1 -> 2 -> 3 -> 2.
- 1 -> 2 -> 1 -> 2.
- 1 -> 0 -> 1 -> 2.
It can be proven that no other way is possible, so we return 3.

Example 2:

Input: startPos = 2, endPos = 5, k = 10
Output: 0
Explanation: It is impossible to reach position 5 from position 2 in exactly 10 steps.

Constraints:

    1 <= startPos, endPos, k <= 1000
*/

struct Solution;

impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        use std::collections::HashMap;
        const MOD: i32 = 1000000007;

        fn dfs(curr_pos: i32, end_pos: i32, k: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
            if k == 0 {
                if curr_pos == end_pos { 1 } else { 0 }
            } else if let Some(n_ways) = memo.get(&(curr_pos, k)) {
                *n_ways
            } else {
                let v1 = dfs(curr_pos + 1, end_pos, k - 1, memo);
                let v2 = dfs(curr_pos - 1, end_pos, k - 1, memo);
                let rez = (v1 + v2) % MOD;
                memo.insert((curr_pos, k), rez);
                rez
            }
        }

        dfs(start_pos, end_pos, k, &mut HashMap::new())
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_ways(1, 2, 3), 3);
    assert_eq!(Solution::number_of_ways(2, 5, 10), 0);
}
