#![allow(dead_code)]

// 909. Snakes and Ladders
// https://leetcode.com/problems/snakes-and-ladders/
// https://leetcode.cn/problems/snakes-and-ladders/
//
// You are given an n x n integer matrix board where the cells are labeled from 1 to n2 in a Boustrophedon
// style starting from the bottom left of the board (i.e. board[n - 1][0]) and alternating direction each row.
//
// You start on square 1 of the board. In each move, starting from square curr, do the following:
//
// Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n2)].
// This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 6 destinations, regardless of the size of the board.
// If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you move to next.
// The game ends when you reach the square n2.
// A board square on row r and column c has a snake or ladder if board[r][c] != -1.
// The destination of that snake or ladder is board[r][c]. Squares 1 and n2 do not have a snake or ladder.
//
// Note that you only take a snake or ladder at most once per move. If the destination to a snake or
// ladder is the start of another snake or ladder, you do not follow the subsequent snake or ladder.
//
// For example, suppose the board is [[-1,4],[-1,3]], and on the first move, your destination square is 2.
// You follow the ladder to square 3, but do not follow the subsequent ladder to 4.
//
// Return the least number of moves required to reach the square n2. If it is not possible to reach the square, return -1.
//
// Example 1:
//
// Input: board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
// Output: 4
// Explanation:
// In the beginning, you start at square 1 (at row 5, column 0).
// You decide to move to square 2 and must take the ladder to square 15.
// You then decide to move to square 17 and must take the snake to square 13.
// You then decide to move to square 14 and must take the ladder to square 35.
// You then decide to move to square 36, ending the game.
// This is the lowest possible number of moves to reach the last square, so return 4.
//
// Example 2:
//
// Input: board = [[-1,-1],[-1,3]]
// Output: 1
//
// Constraints:
//
// - n == board.length == board[i].length
// - 2 <= n <= 20
// - grid[i][j] is either -1 or in the range [1, n2].
// - The squares labeled 1 and n2 do not have any ladders or snakes.
//

struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (board.len(), board[0].len());
        let mut q = vec![];
        let mut steps = 0;
        let mut flag = vec![0; n * n + 1];

        q.push(1);
        while !q.is_empty() {
            let mut temp = vec![];
            while let Some(u) = q.pop() {
                if u == n * n {
                    return steps;
                }
                let mut v = u + 1;
                while v <= u + 6 && v <= n * n {
                    let (i, mut j) = (m - 1 - (v - 1) / n, (v - 1) % n);
                    if !(m - 1 - i).is_multiple_of(2) {
                        j = n - 1 - j;
                    }
                    let w = board[i][j];
                    if w != -1 {
                        if flag[w as usize] == 0 {
                            temp.push(w as usize);
                        }
                        flag[w as usize] = 1;
                    } else {
                        if flag[v] == 0 {
                            temp.push(v);
                        }
                        flag[v] = 1;
                    }
                    v += 1;
                }
            }
            q = temp;
            steps += 1;
        }

        -1
    }
}

#[test]
fn test() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    assert_eq!(Solution::snakes_and_ladders(board), 4);

    let board = vec![vec![-1, -1], vec![-1, 3]];
    assert_eq!(Solution::snakes_and_ladders(board), 1);
}
