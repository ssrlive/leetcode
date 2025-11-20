#![allow(dead_code)]

// 3694. Distinct Points Reachable After Substring Removal
// https://leetcode.com/problems/distinct-points-reachable-after-substring-removal/
// https://leetcode.cn/problems/distinct-points-reachable-after-substring-removal/
//
// Medium
//
// You are given a string s consisting of characters 'U', 'D', 'L', and 'R', representing moves on an infinite 2D Cartesian grid.
//
// 'U': Move from (x, y) to (x, y + 1).
// 'D': Move from (x, y) to (x, y - 1).
// 'L': Move from (x, y) to (x - 1, y).
// 'R': Move from (x, y) to (x + 1, y).
// You are also given a positive integer k.
//
// You must choose and remove exactly one contiguous substring of length k from s. Then, start from coordinate (0, 0) and perform the remaining moves in order.
//
// Return an integer denoting the number of distinct final coordinates reachable.
//
// Example 1:
//
// Input: s = "LUL", k = 1
//
// Output: 2
//
// Explanation:
//
// After removing a substring of length 1, s can be "UL", "LL" or "LU". Following these moves, the final coordinates will be (-1, 1), (-2, 0) and (-1, 1) respectively. There are two distinct points (-1, 1) and (-2, 0) so the answer is 2.
//
// Example 2:
//
// Input: s = "UDLR", k = 4
//
// Output: 1
//
// Explanation:
//
// After removing a substring of length 4, s can only be the empty string. The final coordinates will be (0, 0). There is only one distinct point (0, 0) so the answer is 1.
//
// Example 3:
//
// Input: s = "UU", k = 1
//
// Output: 1
//
// Explanation:
//
// After removing a substring of length 1, s becomes "U", which always ends at (0, 1), so there is only one distinct final coordinate.
//
// Constraints:
//
// 1 <= s.length <= 10^5
// s consists of only 'U', 'D', 'L', and 'R'.
// 1 <= k <= s.length
//

struct Solution;

impl Solution {
    pub fn distinct_points(s: String, k: i32) -> i32 {
        use std::collections::HashSet;
        if k as usize == s.len() {
            return 1;
        }

        let mut pfx: Vec<(i32, i32)> = vec![];
        let mut x = 0;
        let mut y = 0;
        pfx.push((x, y));

        for c in s.chars() {
            match c {
                'U' => y += 1,
                'R' => x += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                _ => (),
            };
            pfx.push((x, y));
        }

        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let dest = pfx[pfx.len() - 1];
        let mut r = k as usize;
        let mut l = 0;
        while r < s.len() {
            let pr = pfx[r];
            let pl = pfx[l];
            let diff_x = pr.0 - pl.0;
            let diff_y = pr.1 - pl.1;
            let x = dest.0 - diff_x;
            let y = dest.1 - diff_y;
            visited.insert((x, y));
            // advance
            r += 1;
            l += 1;
        }

        // special case: last substring (goes until the end)
        let dest = pfx[pfx.len() - 1 - k as usize];
        visited.insert(dest);

        visited.len() as i32
    }
}

#[test]
fn test() {
    let s1 = "LUL".to_string();
    let k1 = 1;
    let result1 = Solution::distinct_points(s1, k1);
    assert_eq!(result1, 2);

    let s2 = "UDLR".to_string();
    let k2 = 4;
    let result2 = Solution::distinct_points(s2, k2);
    assert_eq!(result2, 1);

    let s3 = "UU".to_string();
    let k3 = 1;
    let result3 = Solution::distinct_points(s3, k3);
    assert_eq!(result3, 1);
}
