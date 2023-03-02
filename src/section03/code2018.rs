#![allow(dead_code)]

/*

// 2018. Check if Word Can Be Placed In Crossword
// https://leetcode.com/problems/check-if-word-can-be-placed-in-crossword/
// https://leetcode.cn/problems/check-if-word-can-be-placed-in-crossword/
//
// Medium
//
// You are given an m x n matrix board, representing the current state of a crossword puzzle. The crossword contains lowercase English letters (from solved words), ' ' to represent any empty cells, and '#' to represent any blocked cells.

A word can be placed horizontally (left to right or right to left) or vertically (top to bottom or bottom to top) in the board if:

    It does not occupy a cell containing the character '#'.
    The cell each letter is placed in must either be ' ' (empty) or match the letter already on the board.
    There must not be any empty cells ' ' or other lowercase letters directly left or right of the word if the word was placed horizontally.
    There must not be any empty cells ' ' or other lowercase letters directly above or below the word if the word was placed vertically.

Given a string word, return true if word can be placed in board, or false otherwise.

Example 1:

Input: board = [["#", " ", "#"], [" ", " ", "#"], ["#", "c", " "]], word = "abc"
Output: true
Explanation: The word "abc" can be placed as shown above (top to bottom).

Example 2:

Input: board = [[" ", "#", "a"], [" ", "#", "c"], [" ", "#", "a"]], word = "ac"
Output: false
Explanation: It is impossible to place the word because there will always be a space/letter above or below it.

Example 3:

Input: board = [["#", " ", "#"], [" ", " ", "#"], ["#", " ", "c"]], word = "ca"
Output: true
Explanation: The word "ca" can be placed as shown above (right to left).

Constraints:

    m == board.length
    n == board[i].length
    1 <= m * n <= 2 * 10^5
    board[i][j] will be ' ', '#', or a lowercase English letter.
    1 <= word.length <= max(m, n)
    word will contain only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        fn check(board: &Vec<Vec<char>>, word: &[char]) -> bool {
            let m = board.len();
            let n = board[0].len();
            let mut h = vec![vec![-1; n]; m];
            for i in 0..m {
                for j in 0..n {
                    if j == 0 || board[i][j - 1] == '#' || h[i][j - 1] != -1 {
                        let pos = if j == 0 { 0 } else { (h[i][j - 1] + 1) as usize };
                        if board[i][j] == ' ' || board[i][j] == word[pos] {
                            if pos == word.len() - 1 {
                                if j == n - 1 || board[i][j + 1] == '#' {
                                    return true;
                                }
                            } else {
                                h[i][j] = pos as i32;
                            }
                        }
                    }
                }
            }
            false
        }

        let m = board.len();
        let n = board[0].len();
        let mut trans = vec![vec![' '; m]; n];
        for i in 0..m {
            for (j, trans_j) in trans.iter_mut().enumerate() {
                trans_j[i] = board[i][j];
            }
        }
        let rev_word = word.chars().rev().collect::<Vec<_>>();
        let word = word.chars().collect::<Vec<_>>();
        check(&board, &word) || check(&board, &rev_word) || check(&trans, &word) || check(&trans, &rev_word)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let board = vec![
        vec!['#', ' ', '#'],
        vec![' ', ' ', '#'],
        vec!['#', 'c', ' '],
    ];
    let word = "abc".to_string();
    assert_eq!(Solution::place_word_in_crossword(board, word), true);

    #[rustfmt::skip]
    let board = vec![
        vec![' ', '#', 'a'],
        vec![' ', '#', 'c'],
        vec![' ', '#', 'a'],
    ];
    let word = "ac".to_string();
    assert_eq!(Solution::place_word_in_crossword(board, word), false);

    #[rustfmt::skip]
    let board = vec![
        vec!['#', ' ', '#'],
        vec![' ', ' ', '#'],
        vec!['#', ' ', 'c'],
    ];
    let word = "ca".to_string();
    assert_eq!(Solution::place_word_in_crossword(board, word), true);
}
