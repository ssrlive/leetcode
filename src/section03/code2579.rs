#![allow(dead_code)]

/*

// 2579. Count Total Number of Colored Cells
// https://leetcode.com/problems/count-total-number-of-colored-cells/
// https://leetcode.cn/problems/count-total-number-of-colored-cells/
//
// Medium
//
// There exists an infinitely large two-dimensional grid of uncolored unit cells. You are given a positive integer n, indicating that you must do the following routine for n minutes:

    At the first minute, color any arbitrary unit cell blue.
    Every minute thereafter, color blue every uncolored cell that touches a blue cell.

Below is a pictorial representation of the state of the grid after minutes 1, 2, and 3.

Return the number of colored cells at the end of n minutes.

Example 1:

Input: n = 1
Output: 1
Explanation: After 1 minute, there is only 1 blue cell, so we return 1.

Example 2:

Input: n = 2
Output: 5
Explanation: After 2 minutes, there are 4 colored cells on the boundary and 1 in the center, so we return 5.

Constraints:

    1 <= n <= 10^5
*/

struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        (1..=n as usize).fold(1, |init, val| init + 4 * (val as i64 - 1))
    }
}

#[test]
fn test() {
    assert_eq!(Solution::colored_cells(1), 1);
    assert_eq!(Solution::colored_cells(2), 5);
    assert_eq!(Solution::colored_cells(3), 13);
    assert_eq!(Solution::colored_cells(4), 25);
}
