#![allow(dead_code)]

// 3208. Alternating Groups II
// https://leetcode.com/problems/alternating-groups-ii/
// https://leetcode.cn/problems/alternating-groups-ii/
//
// Medium
//
// There is a circle of red and blue tiles. You are given an array of integers colors and an integer k.
// The color of tile i is represented by colors[i]:
//
//     colors[i] == 0 means that tile i is red.
//     colors[i] == 1 means that tile i is blue.
//
// An alternating group is every k contiguous tiles in the circle with alternating colors
// (each tile in the group except the first and last one has a different color from its left and right tiles).
//
// Return the number of alternating groups.
//
// Note that since colors represents a circle, the first and the last tiles are considered to be next to each other.
//
// Example 1:
//
// Input: colors = [0,1,0,1,0], k = 3
//
// Output: 3
//
// Explanation:
//
// Alternating groups:
//
// Example 2:
//
// Input: colors = [0,1,0,0,1,0,1], k = 6
//
// Output: 2
//
// Explanation:
//
// Alternating groups:
//
// Example 3:
//
// Input: colors = [1,1,0,1], k = 4
//
// Output: 0
//
// Explanation:
//
// Constraints:
//
//     3 <= colors.length <= 10^5
//     0 <= colors[i] <= 1
//     3 <= k <= colors.length
//

struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let (mut result, mut prev, mut count) = (0, colors[0], 1);
        for &colors_idx in colors.iter().skip(1) {
            count = if colors_idx != prev { count + 1 } else { 1 };
            if count >= k {
                result += 1;
            }
            prev = colors_idx;
        }
        for &colors_idx in colors.iter().take(k as usize - 1) {
            count = if colors_idx != prev { count + 1 } else { 1 };
            if count >= k {
                result += 1;
            }
            prev = colors_idx;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3), 3);
    assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6), 2);
    assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4), 0);
}
