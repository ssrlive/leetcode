#![allow(dead_code)]

// 773. Sliding Puzzle
// https://leetcode.com/problems/sliding-puzzle/
// https://leetcode.cn/problems/sliding-puzzle/
//
// On an 2 x 3 board, there are five tiles labeled from 1 to 5, and an empty square represented by 0.
// A move consists of choosing 0 and a 4-directionally adjacent number and swapping it.
//
// The state of the board is solved if and only if the board is [[1,2,3],[4,5,0]].
//
// Given the puzzle board board, return the least number of moves required so that the state of the board is solved.
// If it is impossible for the state of the board to be solved, return -1.
//
// Example 1:
//
// Input: board = [[1,2,3],[4,0,5]]
// Output: 1
// Explanation: Swap the 0 and the 5 in one move.
//
// Example 2:
//
// Input: board = [[1,2,3],[5,4,0]]
// Output: -1
// Explanation: No number of moves will make the board solved.
//
// Example 3:
//
// Input: board = [[4,1,2],[5,0,3]]
// Output: 5
// Explanation: 5 is the smallest number of moves that solves the board.
// An example path:
// After move 0: [[4,1,2],[5,0,3]]
// After move 1: [[4,1,2],[0,5,3]]
// After move 2: [[0,1,2],[4,5,3]]
// After move 3: [[1,0,2],[4,5,3]]
// After move 4: [[1,2,0],[4,5,3]]
// After move 5: [[1,2,3],[4,5,0]]
//
// Constraints:
//
// - board.length == 2
// - board[i].length == 3
// - 0 <= board[i][j] <= 5
// - Each value board[i][j] is unique.
//

struct Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        fn string_swap(s: &str, from_idx: usize, to_idx: usize) -> String {
            let mut chars: Vec<_> = s.chars().collect();
            chars.swap(from_idx, to_idx);
            chars.into_iter().collect()
        }

        fn _sliding_puzzle(board: Vec<Vec<i32>>) -> Option<i32> {
            let goal = "123450".to_string();
            let moves = [vec![1, 3], vec![0, 2, 4], vec![1, 5], vec![0, 4], vec![1, 3, 5], vec![2, 4]];
            let mut cr = String::new();
            for row in board.iter() {
                for &val in row.iter() {
                    cr.push_str(&val.to_string());
                }
            }
            let mut q = std::collections::VecDeque::new();
            q.push_back((cr.clone(), 0));
            let mut vis = std::collections::HashSet::new();
            vis.insert(cr);
            while !q.is_empty() {
                let t = q.pop_front()?;
                let nw = t.0;
                let ans = t.1;
                if nw == goal {
                    return Some(ans);
                }
                let inx = nw.find('0')?;
                for i in &moves[inx] {
                    let st = string_swap(&nw, *i, inx);
                    if !vis.contains(&st) {
                        vis.insert(st.clone());
                        q.push_back((st, ans + 1));
                    }
                }
            }
            Some(-1)
        }

        _sliding_puzzle(board).unwrap_or(-1)
    }
}

#[test]
fn test() {
    let board = vec![vec![1, 2, 3], vec![4, 0, 5]];
    assert_eq!(Solution::sliding_puzzle(board), 1);
    let board = vec![vec![1, 2, 3], vec![5, 4, 0]];
    assert_eq!(Solution::sliding_puzzle(board), -1);
    let board = vec![vec![4, 1, 2], vec![5, 0, 3]];
    assert_eq!(Solution::sliding_puzzle(board), 5);
}
