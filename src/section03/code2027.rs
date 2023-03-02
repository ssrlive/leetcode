#![allow(dead_code)]

/*

// 2027. Minimum Moves to Convert String
// https://leetcode.com/problems/minimum-moves-to-convert-string/
// https://leetcode.cn/problems/minimum-moves-to-convert-string/
//
// Easy
//
// You are given a string s consisting of n characters which are either 'X' or 'O'.

A move is defined as selecting three consecutive characters of s and converting them to 'O'. Note that if a move is applied to the character 'O', it will stay the same.

Return the minimum number of moves required so that all the characters of s are converted to 'O'.

Example 1:

Input: s = "XXX"
Output: 1
Explanation: XXX -> OOO
We select all the 3 characters and convert them in one move.

Example 2:

Input: s = "XXOX"
Output: 2
Explanation: XXOX -> OOOX -> OOOO
We select the first 3 characters in the first move, and convert them to 'O'.
Then we select the last 3 characters and convert them so that the final string contains all 'O's.

Example 3:

Input: s = "OOOO"
Output: 0
Explanation: There are no 'X's in s to convert.

Constraints:

    3 <= s.length <= 1000
    s[i] is either 'X' or 'O'.
*/

struct Solution;

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        s.into_bytes()
            .iter()
            .enumerate()
            .filter(|&(_, b)| *b == b'X')
            .fold((0, 0), |(mut moves, mut next_ind), (ind, _)| {
                if ind >= next_ind {
                    moves += 1;
                    next_ind = ind + 3
                };
                (moves, next_ind)
            })
            .0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_moves("XXX".to_string()), 1);
    assert_eq!(Solution::minimum_moves("XXOX".to_string()), 2);
    assert_eq!(Solution::minimum_moves("OOOO".to_string()), 0);
}
