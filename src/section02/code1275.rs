#![allow(dead_code)]

// 1275. Find Winner on a Tic Tac Toe Game
// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/
// https://leetcode.cn/problems/find-winner-on-a-tic-tac-toe-game/
//
// Easy
//
// Tic-tac-toe is played by two players A and B on a 3 x 3 grid. The rules of Tic-Tac-Toe are:
//
//     Players take turns placing characters into empty squares ' '.
//     The first player A always places 'X' characters, while the second player B always places 'O' characters.
//     'X' and 'O' characters are always placed into empty squares, never on filled ones.
//     The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
//     The game also ends if all squares are non-empty.
//     No more moves can be played if the game is over.
//
// Given a 2D integer array moves where moves[i] = [rowi, coli] indicates that the ith move will
// be played on grid[rowi][coli]. return the winner of the game if it exists (A or B).
// In case the game ends in a draw return "Draw". If there are still movements to play return "Pending".
//
// You can assume that moves is valid (i.e., it follows the rules of Tic-Tac-Toe), the grid is initially empty, and A will play first.
//
// Example 1:
//
// Input: moves = [[0,0],[2,0],[1,1],[2,1],[2,2]]
// Output: "A"
// Explanation: A wins, they always play first.
//
// Example 2:
//
// Input: moves = [[0,0],[1,1],[0,1],[0,2],[1,0],[2,0]]
// Output: "B"
// Explanation: B wins.
//
// Example 3:
//
// Input: moves = [[0,0],[1,1],[2,0],[1,0],[1,2],[2,1],[0,1],[0,2],[2,2]]
// Output: "Draw"
// Explanation: The game ends in a draw since there are no moves to make.
//
// Constraints:
//
// -    1 <= moves.length <= 9
// -    moves[i].length == 2
// -    0 <= rowi, coli <= 2
// -    There are no repeated elements on moves.
// -    moves follow the rules of tic tac toe.
//

struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let moves_len = moves.len();
        if moves_len < 5 {
            return String::from("Pending");
        }
        if let Some(x) = Self::check_straight_line(&moves) {
            return x;
        }
        match moves_len {
            9 => String::from("Draw"),
            _ => String::from("Pending"),
        }
    }

    fn check_sum(sum: i32) -> Option<String> {
        match sum {
            3 => Some(String::from("A")),
            15 => Some(String::from("B")),
            _ => None,
        }
    }

    pub fn check_straight_line(moves: &[Vec<i32>]) -> Option<String> {
        let mut state = [[0; 3]; 3];
        for (n, mv) in moves.iter().enumerate() {
            let (i, j) = (mv[0], mv[1]);
            match n % 2 {
                0 => state[i as usize][j as usize] = 1,
                _ => state[i as usize][j as usize] = 5,
            }
        }
        // Check Horizontal
        for row in state {
            let sum = row.iter().sum::<i32>();
            if let Some(x) = Self::check_sum(sum) {
                return Some(x);
            }
        }
        // Check Vertical
        for i in 0..3 {
            let mut sum = 0;
            for item in &state {
                sum += item[i];
            }
            if let Some(x) = Self::check_sum(sum) {
                return Some(x);
            }
        }
        // Check cross
        let mut sum = 0;
        for (i, j) in [(0, 0), (1, 1), (2, 2)] {
            sum += state[i][j];
        }
        if let Some(x) = Self::check_sum(sum) {
            return Some(x);
        }
        // Check other cross
        let mut sum = 0;
        for (i, j) in [(0, 2), (1, 1), (2, 0)] {
            sum += state[i][j];
        }
        if let Some(x) = Self::check_sum(sum) {
            return Some(x);
        }
        None
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]], "A"),
        (vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![0, 2], vec![1, 0], vec![2, 0]], "B"),
        (
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2],
            ],
            "Draw",
        ),
        (vec![vec![0, 0], vec![1, 1]], "Pending"),
    ];
    for (moves, expect) in cases {
        assert_eq!(Solution::tictactoe(moves), expect);
    }
}
