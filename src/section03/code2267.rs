#![allow(dead_code)]

/*

// 2267. Check if There Is a Valid Parentheses String Path
// https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path/
// https://leetcode.cn/problems/check-if-there-is-a-valid-parentheses-string-path/
//
// Hard
//
// A parentheses string is a non-empty string consisting only of '(' and ')'. It is valid if any of the following conditions is true:

    It is ().
    It can be written as AB (A concatenated with B), where A and B are valid parentheses strings.
    It can be written as (A), where A is a valid parentheses string.

You are given an m x n matrix of parentheses grid. A valid parentheses string path in the grid is a path satisfying all of the following conditions:

    The path starts from the upper left cell (0, 0).
    The path ends at the bottom-right cell (m - 1, n - 1).
    The path only ever moves down or right.
    The resulting parentheses string formed by the path is valid.

Return true if there exists a valid parentheses string path in the grid. Otherwise, return false.

Example 1:

Input: grid = [["(","(","("],[")","(",")"],["(","(",")"],["(","(",")"]]
Output: true
Explanation: The above diagram shows two possible paths that form valid parentheses strings.
The first path shown results in the valid parentheses string "()(())".
The second path shown results in the valid parentheses string "((()))".
Note that there may be other valid parentheses string paths.

Example 2:

Input: grid = [[")",")"],["(","("]]
Output: false
Explanation: The two possible paths form the parentheses strings "))(" and ")((". Since neither of them are valid parentheses strings, we return false.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 100
    grid[i][j] is either '(' or ')'.
*/

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        fn solve(grid: &Vec<Vec<char>>, dp: &mut Vec<Vec<Vec<i32>>>, i: usize, j: usize, k: usize) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            if i == m || j == n {
                return 0;
            }
            let mut k = k as i32;
            k += if grid[i][j] == '(' { 1 } else { -1 };
            if k < 0 {
                return 0;
            }
            let k = k as usize;
            if i == m - 1 && j == n - 1 {
                return if k == 0 { 1 } else { 0 };
            }
            if dp[i][j][k] != -1 {
                return dp[i][j][k];
            }
            let v1 = solve(grid, dp, i + 1, j, k);
            let v2 = solve(grid, dp, i, j + 1, k);
            dp[i][j][k] = if v1 == 1 || v2 == 1 { 1 } else { 0 };
            dp[i][j][k]
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![vec![-1; m + n]; n]; m];
        solve(&grid, &mut dp, 0, 0, 0) == 1
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec!['(', '(', '('],
                vec![')', '(', ')'],
                vec!['(', '(', ')'],
                vec!['(', '(', ')'],
            ],
            true,
        ),
        (vec![vec![')', ')'], vec!['(', '(']], false),
        (vec![vec!['(', ')']], true),
        (vec![vec!['(', ')'], vec!['(', ')']], false),
        (vec![vec!['(', ')'], vec!['(', ')'], vec!['(', ')']], true),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::has_valid_path(grid), expected);
    }
}
