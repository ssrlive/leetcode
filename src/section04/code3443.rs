#![allow(dead_code)]

// 3443. Maximum Manhattan Distance After K Changes
// https://leetcode.com/problems/maximum-manhattan-distance-after-k-changes/
// https://leetcode.cn/problems/maximum-manhattan-distance-after-k-changes/
//
// Medium
//
// You are given a string s consisting of the characters 'N', 'S', 'E', and 'W', where s[i] indicates movements in an infinite grid:
//
//     'N' : Move north by 1 unit.
//     'S' : Move south by 1 unit.
//     'E' : Move east by 1 unit.
//     'W' : Move west by 1 unit.
//
// Initially, you are at the origin (0, 0). You can change at most k characters to any of the four directions.
//
// Find the maximum Manhattan distance from the origin that can be achieved at any time while performing the movements in order.
// The Manhattan Distance between two cells (xi, yi) and (xj, yj) is |xi - xj| + |yi - yj|.
//
// Example 1:
//
// Input: s = "NWSE", k = 1
//
// Output: 3
//
// Explanation:
//
// Change s[2] from 'S' to 'N'. The string s becomes "NWNE".
// Movement	Position (x, y)	Manhattan Distance	Maximum
// s[0] == 'N'	(0, 1)	0 + 1 = 1	1
// s[1] == 'W'	(-1, 1)	1 + 1 = 2	2
// s[2] == 'N'	(-1, 2)	1 + 2 = 3	3
// s[3] == 'E'	(0, 2)	0 + 2 = 2	3
//
// The maximum Manhattan distance from the origin that can be achieved is 3. Hence, 3 is the output.
//
// Example 2:
//
// Input: s = "NSWWEW", k = 3
//
// Output: 6
//
// Explanation:
//
// Change s[1] from 'S' to 'N', and s[4] from 'E' to 'W'. The string s becomes "NNWWWW".
//
// The maximum Manhattan distance from the origin that can be achieved is 6. Hence, 6 is the output.
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     0 <= k <= s.length
//     s consists of only 'N', 'S', 'E', and 'W'.
//

struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let (mut res, mut hor, mut ver) = (1_i32, 0_i32, 0_i32);
        for (i, c) in s.chars().enumerate() {
            ver += if c == 'N' {
                1
            } else if c == 'S' {
                -1
            } else {
                0
            };
            hor += if c == 'W' {
                1
            } else if c == 'E' {
                -1
            } else {
                0
            };
            res = res.max((hor.abs() + ver.abs() + 2 * k).min(i as i32 + 1));
        }
        res
    }
}

#[test]
fn test() {
    let s = "NWSE".to_string();
    let k = 1;
    let res = 3;
    assert_eq!(Solution::max_distance(s, k), res);

    let s = "NSWWEW".to_string();
    let k = 3;
    let res = 6;
    assert_eq!(Solution::max_distance(s, k), res);
}
