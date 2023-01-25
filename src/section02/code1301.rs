#![allow(dead_code)]

// 1301. Number of Paths with Max Score
// https://leetcode.com/problems/number-of-paths-with-max-score/
// https://leetcode.cn/problems/number-of-paths-with-max-score/
//
// Hard
//
// You are given a square board of characters. You can move on the board starting at the bottom right square marked with the character 'S'.
//
// You need to reach the top left square marked with the character 'E'. The rest of the squares are labeled either
// with a numeric character 1, 2, ..., 9 or with an obstacle 'X'. In one move you can go up, left or up-left (diagonally)
// only if there is no obstacle there.
//
// Return a list of two integers: the first integer is the maximum sum of numeric characters you can collect,
// and the second is the number of such paths that you can take to get that maximum sum, taken modulo 10^9 + 7.
//
// In case there is no path, return [0, 0].
//
// Example 1:
//
// Input: board = ["E23","2X2","12S"]
// Output: [7,1]
//
// Example 2:
//
// Input: board = ["E12","1X1","21S"]
// Output: [4,2]
//
// Example 3:
//
// Input: board = ["E11","XXX","11S"]
// Output: [0,0]
//
// Constraints:
//
//     2 <= board.length == board[i].length <= 100
//

struct Solution;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let dirs = vec![vec![1, 0], vec![0, 1], vec![1, 1]];
        let sz = board.len();
        let mut score = vec![vec![0; sz + 1]; sz + 1];
        let mut paths = vec![vec![0; sz + 1]; sz + 1];
        let mut board = board;
        board[0].replace_range(..1, "0");
        board[sz - 1].replace_range(sz - 1.., "0");
        paths[0][0] = 1;
        for i in 1..=sz {
            for j in 1..=sz {
                let board_ij = board[i - 1].chars().nth(j - 1).unwrap();
                if board_ij == 'X' {
                    continue;
                }
                for d in &dirs {
                    let i1 = i - d[0];
                    let j1 = j - d[1];
                    let val = score[i1][j1] + (board_ij as i32 - '0' as i32);
                    if score[i][j] <= val && paths[i1][j1] > 0 {
                        paths[i][j] = (if score[i][j] == val { paths[i][j] } else { 0 } + paths[i1][j1]) % 1000000007;
                        score[i][j] = val;
                    }
                }
            }
        }
        let val = paths[sz][sz];
        vec![if val != 0 { score[sz][sz] } else { 0 }, val]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["E23", "2X2", "12S"], vec![7, 1]),
        (vec!["E12", "1X1", "21S"], vec![4, 2]),
        (vec!["E11", "XXX", "11S"], vec![0, 0]),
    ];
    for (board, expect) in cases {
        let board: Vec<String> = board.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::paths_with_max_score(board), expect);
    }
}
