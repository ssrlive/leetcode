#![allow(dead_code)]

// 79. Word Search
// https://leetcode.com/problems/word-search/
// https://leetcode.cn/problems/word-search/
//
// Given an m x n grid of characters board and a string word, return true if word exists in the grid.
//
// The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
//
// Example 1:
//
// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// Output: true
//
// Example 2:
//
// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
// Output: true
//
// Example 3:
//
// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
// Output: false
//
// Constraints:
//
// - m == board.length
// - n = board[i].length
// - 1 <= m, n <= 6
// - 1 <= word.length <= 15
// - board and word consists of only lowercase and uppercase English letters.
//
// Follow up: Could you use search pruning to make your solution faster with a larger board?
//

struct Solution;

impl Solution {
    pub fn exist(b: Vec<Vec<char>>, word: String) -> bool {
        // optimisation of checking if there are enough character to make the word before using DFS
        let mut a = [0; 60];
        for s in b.iter() {
            for &x in s.iter() {
                a[x as usize - 64] += 1;
            }
        }
        for x in word.chars() {
            a[x as usize - 64] -= 1;
            if a[x as usize - 64] < 0 {
                return false;
            }
        }

        let (w, n, m) = (word.chars().collect::<Vec<_>>(), b.len(), b[0].len());
        let mut f = vec![vec![true; m + 2]; n + 2];

        for item in f.iter_mut() {
            item[0] = false;
            item[m + 1] = false;
        }
        for j in 0..m + 2 {
            f[0][j] = false;
            f[n + 1][j] = false;
        }

        fn rec(b: &Vec<Vec<char>>, w: &Vec<char>, i: usize, j: usize, k: usize, f: &mut Vec<Vec<bool>>) -> bool {
            if k == w.len() {
                return true;
            }
            f[i + 1][j + 1] = false;
            if f[i][j + 1] && b[i - 1][j] == w[k] && rec(b, w, i - 1, j, k + 1, f) {
                return true;
            }
            if f[i + 1][j] && b[i][j - 1] == w[k] && rec(b, w, i, j - 1, k + 1, f) {
                return true;
            }
            if f[i + 2][j + 1] && b[i + 1][j] == w[k] && rec(b, w, i + 1, j, k + 1, f) {
                return true;
            }
            if f[i + 1][j + 2] && b[i][j + 1] == w[k] && rec(b, w, i, j + 1, k + 1, f) {
                return true;
            }

            f[i + 1][j + 1] = true;
            false
        }

        for i in 0..n {
            for j in 0..m {
                if b[i][j] == w[0] && rec(&b, &w, i, j, 1, &mut f) {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
            "ABCCED",
            true,
        ),
        (
            vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
            "SEE",
            true,
        ),
        (
            vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
            "ABCB",
            false,
        ),
        (
            vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'E', 'S'], vec!['A', 'D', 'E', 'E']],
            "ABCESEEEFS",
            true,
        ),
    ];
    for (b, word, expected) in cases {
        assert_eq!(Solution::exist(b, word.to_string()), expected);
    }
}
