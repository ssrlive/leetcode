#![allow(dead_code)]

// 3529. Count Cells in Overlapping Horizontal and Vertical Substrings
// https://leetcode.com/problems/count-cells-in-overlapping-horizontal-and-vertical-substrings/
// https://leetcode.cn/problems/count-cells-in-overlapping-horizontal-and-vertical-substrings/
//
// Medium
//
// You are given an m x n matrix grid consisting of characters and a string pattern.
//
// A horizontal substring is a contiguous sequence of characters read from left to right. If the end of a row is reached
// before the substring is complete, it wraps to the first column of the next row and continues as needed.
// You do not wrap from the bottom row back to the top.
//
// A vertical substring is a contiguous sequence of characters read from top to bottom. If the bottom of a column is
// reached before the substring is complete, it wraps to the first row of the next column and continues as needed.
// You do not wrap from the last column back to the first.
//
// Count the number of cells in the matrix that satisfy the following condition:
//
// - The cell must be part of at least one horizontal substring and at least one vertical substring,
//   where both substrings are equal to the given pattern.
//
// Return the count of these cells.
//
// Example 1:
//
// Input: grid = [["a","a","c","c"],["b","b","b","c"],["a","a","b","a"],["c","a","a","c"],["a","a","c","c"]], pattern = "abaca"
//
// Output: 1
//
// Explanation:
//
// The pattern "abaca" appears once as a horizontal substring (colored blue) and once as a vertical
// substring (colored red), intersecting at one cell (colored purple).
//
// Example 2:
//
// Input: grid = [["c","a","a","a"],["a","a","b","a"],["b","b","a","a"],["a","a","b","a"]], pattern = "aba"
//
// Output: 4
//
// Explanation:
//
// The cells colored above are all part of at least one horizontal and one vertical substring matching the pattern "aba".
//
// Example 3:
//
// Input: grid = [["a"]], pattern = "a"
//
// Output: 1
//
// Constraints:
//
//     m == grid.length
//     n == grid[i].length
//     1 <= m, n <= 1000
//     1 <= m * n <= 105
//     1 <= pattern.length <= m * n
//     grid and pattern consist of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn count_cells(grid: Vec<Vec<char>>, pattern: String) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        const MOD: i64 = i32::MAX as i64;
        // rabin karp to detect pattern
        let mut target = 0i64;
        for &c in pattern.as_bytes() {
            let c = c as i64 - 97;
            target = ((target * 26) % MOD + c) % MOD;
        }
        let mut pow = 1;
        for _ in 0..pattern.len() - 1 {
            pow = (pow * 26) % MOD;
        }
        let mut grid_pattern = vec![0; n * m];

        // run horizontal
        let mut last = 0;
        let mut cur = 0;
        for i in 0..m * n {
            let (r, c) = (i / n, i % n);
            let c = grid[r][c] as i64 - 97;
            if i < pattern.len() {
                cur = ((cur * 26) % MOD + c) % MOD;
            } else {
                let pi = i - pattern.len();
                let (pr, pc) = (pi / n, pi % n);
                let pc = grid[pr][pc] as i64 - 97;

                cur = ((cur - (pow * pc) % MOD + MOD) * 26) % MOD + c;
            }
            if i >= pattern.len() - 1 && cur == target {
                let start = (i as i32 - pattern.len() as i32 + 1).max(last as i32) as usize;
                for grid_pattern_j in grid_pattern.iter_mut().take(i + 1).skip(start) {
                    *grid_pattern_j = 1;
                }
                last = i;
            }
        }

        // run vertical
        let mut last = 0;
        let mut cur = 0;
        for i in 0..m * n {
            let (r, c) = (i % m, i / m);
            let c = grid[r][c] as i64 - 97;
            if i < pattern.len() {
                cur = ((cur * 26) % MOD + c) % MOD;
            } else {
                let pi = i - pattern.len();
                let (pr, pc) = (pi % m, pi / m);
                let pc = grid[pr][pc] as i64 - 97;

                cur = ((cur - (pow * pc) % MOD + MOD) * 26) % MOD + c;
            }
            if i >= pattern.len() - 1 && cur == target {
                let start = (i as i32 - pattern.len() as i32 + 1).max(last as i32) as usize;
                for j in start..=i {
                    let (r, c) = (j % m, j / m);
                    let j = r * n + c;
                    grid_pattern[j] |= 2;
                }
                last = i;
            }
        }

        // check if row and col is matching
        grid_pattern.iter().map(|&x| if x == 3 { 1 } else { 0 }).sum()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_cells(
            vec![
                vec!['a', 'a', 'c', 'c'],
                vec!['b', 'b', 'b', 'c'],
                vec!['a', 'a', 'b', 'a'],
                vec!['c', 'a', 'a', 'c'],
                vec!['a', 'a', 'c', 'c']
            ],
            "abaca".to_string()
        ),
        1
    );
    assert_eq!(
        Solution::count_cells(
            vec![
                vec!['c', 'a', 'a', 'a'],
                vec!['a', 'a', 'b', 'a'],
                vec!['b', 'b', 'a', 'a'],
                vec!['a', 'a', 'b', 'a']
            ],
            "aba".to_string()
        ),
        4
    );
    assert_eq!(Solution::count_cells(vec![vec!['a']], "a".to_string()), 1);
}
