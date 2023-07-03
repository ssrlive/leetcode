#![allow(dead_code)]

// 1222. Queens That Can Attack the King
// https://leetcode.com/problems/queens-that-can-attack-the-king/
// https://leetcode.cn/problems/queens-that-can-attack-the-king/
//
// Medium
//
// On a 0-indexed 8 x 8 chessboard, there can be multiple black queens ad one white king.
//
// You are given a 2D integer array queens where queens[i] = [xQueeni, yQueeni] represents the position of the ith black queen on the chessboard. You are also given an integer array king of length 2 where king = [xKing, yKing] represents the position of the white king.
//
// Return the coordinates of the black queens that can directly attack the king. You may return the answer in any order.
//
// Example 1:
//
// Input: queens = [[0,1],[1,0],[4,0],[0,4],[3,3],[2,4]], king = [0,0]
// Output: [[0,1],[1,0],[3,3]]
// Explanation: The diagram above shows the three queens that can directly attack the king and the three queens that cannot attack the king (i.e., marked with red dashes).
//
// Example 2:
//
// Input: queens = [[0,0],[1,1],[2,2],[3,4],[3,5],[4,4],[4,5]], king = [3,3]
// Output: [[2,2],[3,4],[4,4]]
// Explanation: The diagram above shows the three queens that can directly attack the king and the three queens that cannot attack the king (i.e., marked with red dashes).
//
// Constraints:
//
// -    1 <= queens.length < 64
// -    queens[i].length == king.length == 2
// -    0 <= xQueeni, yQueeni, xKing, yKing < 8
// -    All the given positions are unique.
//

struct Solution;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::*;
        let mut result = vec![];
        let mut set = HashSet::new();
        for arr in queens {
            set.insert((arr[0], arr[1]));
        }

        let p = [-1, 0, 1];
        for i in 0..3 {
            for j in 0..3 {
                let x = p[i];
                let y = p[j];
                if x == 0 && y == 0 {
                    continue;
                }

                let mut r = king[0];
                let mut c = king[1];
                while 0 <= r && 0 <= c && r < 8 && c < 8 {
                    if set.contains(&(r, c)) {
                        result.push(vec![r, c]);
                        break;
                    }
                    r += x;
                    c += y;
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![0, 1], vec![1, 0], vec![4, 0], vec![0, 4], vec![3, 3], vec![2, 4]],
            vec![0, 0],
            vec![vec![0, 1], vec![1, 0], vec![3, 3]],
        ),
        (
            vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 4], vec![3, 5], vec![4, 4], vec![4, 5]],
            vec![3, 3],
            vec![vec![2, 2], vec![3, 4], vec![4, 4]],
        ),
    ];
    for (queens, king, e) in cases {
        assert_eq!(Solution::queens_attackthe_king(queens, king), e);
    }
}
