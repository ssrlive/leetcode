#![allow(dead_code)]

/*

// 2147. Number of Ways to Divide a Long Corridor
// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/
// https://leetcode.cn/problems/number-of-ways-to-divide-a-long-corridor/
//
// Hard
//
// Along a long library corridor, there is a line of seats and decorative plants. You are given a 0-indexed string corridor of length n consisting of letters 'S' and 'P' where each 'S' represents a seat and each 'P' represents a plant.

One room divider has already been installed to the left of index 0, and another to the right of index n - 1. Additional room dividers can be installed. For each position between indices i - 1 and i (1 <= i <= n - 1), at most one divider can be installed.

Divide the corridor into non-overlapping sections, where each section has exactly two seats with any number of plants. There may be multiple ways to perform the division. Two ways are different if there is a position with a room divider installed in the first way but not in the second way.

Return the number of ways to divide the corridor. Since the answer may be very large, return it modulo 109 + 7. If there is no way, return 0.

Example 1:

Input: corridor = "SSPPSPS"
Output: 3
Explanation: There are 3 different ways to divide the corridor.
The black bars in the above image indicate the two room dividers already installed.
Note that in each of the ways, each section has exactly two seats.

Example 2:

Input: corridor = "PPSPSP"
Output: 1
Explanation: There is only 1 way to divide the corridor, by not installing any additional dividers.
Installing any would create some section that does not have exactly two seats.

Example 3:

Input: corridor = "S"
Output: 0
Explanation: There is no way to divide the corridor because there will always be a section that does not have exactly two seats.

Constraints:

    n == corridor.length
    1 <= n <= 10^5
    corridor[i] is either 'S' or 'P'.
*/

struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut res = 1;
        let mut j = 0;
        let mut k = 0;
        let mod_ = 1_000_000_007;
        for (i, c) in corridor.chars().enumerate() {
            if c == 'S' {
                k += 1;
                if k > 2 && k % 2 == 1 {
                    res = res * (i as i64 - j) % mod_;
                }
                j = i as i64;
            }
        }
        if k % 2 == 0 && k > 0 {
            res as _
        } else {
            0
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
    assert_eq!(Solution::number_of_ways("PPSPSP".to_string()), 1);
    assert_eq!(Solution::number_of_ways("S".to_string()), 0);
}
