#![allow(dead_code)]

/*

// 1931. Painting a Grid With Three Different Colors
// https://leetcode.com/problems/painting-a-grid-with-three-different-colors/
// https://leetcode.cn/problems/painting-a-grid-with-three-different-colors/
//
// Hard
//
// You are given two integers m and n. Consider an m x n grid where each cell is initially white. You can paint each cell red, green, or blue. All cells must be painted.

Return the number of ways to color the grid with no two adjacent cells having the same color. Since the answer can be very large, return it modulo 109 + 7.

Example 1:

Input: m = 1, n = 1
Output: 3
Explanation: The three possible colorings are shown in the image above.

Example 2:

Input: m = 1, n = 2
Output: 6
Explanation: The six possible colorings are shown in the image above.

Example 3:

Input: m = 5, n = 5
Output: 580986

Constraints:

    1 <= m <= 5
    1 <= n <= 1000
*/

struct Solution;

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        use std::collections::HashMap;
        fn dfs_ct(
            m: usize,
            i: usize,
            prev: u16,
            next_cols_cache: &mut HashMap<u16, Vec<u16>>,
            ct_cache: &mut HashMap<(u16, usize), usize>,
        ) -> usize {
            if i == 0 {
                return 1;
            }
            if let Some(cached) = ct_cache.get(&(prev, i)) {
                return *cached;
            }
            let next_cols = match next_cols_cache.get(&prev) {
                Some(a) => a.clone(),
                None => {
                    let mut res = vec![];
                    calc_next_cols(m, prev, 0, 0, &mut res);
                    next_cols_cache.insert(prev, res.clone());
                    res
                }
            };
            let mut ans = 0;
            for col in next_cols {
                ans += dfs_ct(m, i - 1, col, next_cols_cache, ct_cache);
                ans %= 1_000_000_007;
            }
            ct_cache.insert((prev, i), ans);
            ans
        }

        fn calc_next_cols(m: usize, prev: u16, i: usize, acc: u16, res: &mut Vec<u16>) {
            if i == m {
                res.push(acc);
                return;
            }
            let mut r = true;
            let mut g = true;
            let mut b = true;
            let v = (prev >> (i * 2)) & 0b11;
            if v == 1 {
                r = false;
            }
            if v == 2 {
                g = false;
            }
            if v == 3 {
                b = false;
            }
            if i > 0 {
                let v = acc >> ((i - 1) * 2) & 0b11;
                if v == 1 {
                    r = false;
                }
                if v == 2 {
                    g = false;
                }
                if v == 3 {
                    b = false;
                }
            }
            if r {
                calc_next_cols(m, prev, i + 1, acc | 1 << (i * 2), res);
            }
            if g {
                calc_next_cols(m, prev, i + 1, acc | 2 << (i * 2), res);
            }
            if b {
                calc_next_cols(m, prev, i + 1, acc | 3 << (i * 2), res);
            }
        }

        let m = m as usize;
        let n = n as usize;
        dfs_ct(m, n, 0, &mut HashMap::new(), &mut HashMap::new()) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::color_the_grid(1, 1), 3);
    assert_eq!(Solution::color_the_grid(1, 2), 6);
    assert_eq!(Solution::color_the_grid(5, 5), 580986);
}
