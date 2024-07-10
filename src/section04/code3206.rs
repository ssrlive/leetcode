#![allow(dead_code)]

// 3206. Alternating Groups I
// https://leetcode.com/problems/alternating-groups-i/
// https://leetcode.cn/problems/alternating-groups-i/
//
// Easy
//
// There is a circle of red and blue tiles. You are given an array of integers colors. The color of tile i is represented by colors[i]:
//
//     colors[i] == 0 means that tile i is red.
//     colors[i] == 1 means that tile i is blue.
//
// Every 3 contiguous tiles in the circle with alternating colors (the middle tile
// has a different color from its left and right tiles) is called an alternating group.
//
// Return the number of alternating groups.
//
// Note that since colors represents a circle, the first and the last tiles are considered to be next to each other.
//
// Example 1:
//
// Input: colors = [1,1,1]
//
// Output: 0
//
// Explanation:
//
// Example 2:
//
// Input: colors = [0,1,0,0,1]
//
// Output: 3
//
// Explanation:
//
// Alternating groups:
//
// Constraints:
//
//     3 <= colors.length <= 100
//     0 <= colors[i] <= 1
//

struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let mut colors = colors;
        (0..colors.len()).fold(
            {
                colors.push(colors[0]);
                colors.push(colors[1]);
                0
            },
            |mut cnt, i| {
                if colors[i] == 0 && colors[i + 1] == 1 && colors[i + 2] == 0 {
                    cnt += 1;
                }
                if colors[i] == 1 && colors[i + 1] == 0 && colors[i + 2] == 1 {
                    cnt += 1;
                }
                cnt
            },
        )
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 1]), 0);
    assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1]), 3);
}
