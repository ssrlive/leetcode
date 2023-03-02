#![allow(dead_code)]

/*

// 2056. Number of Valid Move Combinations On Chessboard
// https://leetcode.com/problems/number-of-valid-move-combinations-on-chessboard/
// https://leetcode.cn/problems/number-of-valid-move-combinations-on-chessboard/
//
// Hard
//
// There is an 8 x 8 chessboard containing n pieces (rooks, queens, or bishops). You are given a string array pieces of length n, where pieces[i] describes the type (rook, queen, or bishop) of the ith piece. In addition, you are given a 2D integer array positions also of length n, where positions[i] = [ri, ci] indicates that the ith piece is currently at the 1-based coordinate (ri, ci) on the chessboard.

When making a move for a piece, you choose a destination square that the piece will travel toward and stop on.

    A rook can only travel horizontally or vertically from (r, c) to the direction of (r+1, c), (r-1, c), (r, c+1), or (r, c-1).
    A queen can only travel horizontally, vertically, or diagonally from (r, c) to the direction of (r+1, c), (r-1, c), (r, c+1), (r, c-1), (r+1, c+1), (r+1, c-1), (r-1, c+1), (r-1, c-1).
    A bishop can only travel diagonally from (r, c) to the direction of (r+1, c+1), (r+1, c-1), (r-1, c+1), (r-1, c-1).

You must make a move for every piece on the board simultaneously. A move combination consists of all the moves performed on all the given pieces. Every second, each piece will instantaneously travel one square towards their destination if they are not already at it. All pieces start traveling at the 0th second. A move combination is invalid if, at a given time, two or more pieces occupy the same square.

Return the number of valid move combinations​​​​​.

Notes:

    No two pieces will start in the same square.
    You may choose the square a piece is already on as its destination.
    If two pieces are directly adjacent to each other, it is valid for them to move past each other and swap positions in one second.

Example 1:

Input: pieces = ["rook"], positions = [[1,1]]
Output: 15
Explanation: The image above shows the possible squares the piece can move to.

Example 2:

Input: pieces = ["queen"], positions = [[1,1]]
Output: 22
Explanation: The image above shows the possible squares the piece can move to.

Example 3:

Input: pieces = ["bishop"], positions = [[4,3]]
Output: 12
Explanation: The image above shows the possible squares the piece can move to.

Constraints:

    n == pieces.length
    n == positions.length
    1 <= n <= 4
    pieces only contains the strings "rook", "queen", and "bishop".
    There will be at most one queen on the chessboard.
    1 <= xi, yi <= 8
    Each positions[i] is distinct.
*/

struct Solution;

impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        let dir = [[0, 1], [1, 0], [0, -1], [-1, 0], [1, 1], [1, -1], [-1, -1], [-1, 1]];
        let mut b = [[[0; 8]; 8]; 4];

        fn _count_combinations(
            pieces: &[String],
            pos: &[Vec<i32>],
            p: usize,
            dir: &[[i32; 2]],
            b: &mut [[[i32; 8]; 8]; 4],
        ) -> i32 {
            if p >= pieces.len() {
                return 1;
            }
            let i = pos[p][0] - 1;
            let j = pos[p][1] - 1;
            let mut res = 0;
            for d in 0..8 {
                if (d < 4 && pieces[p] == "bishop") || (d >= 4 && pieces[p] == "rook") {
                    continue;
                }
                let mut blocked = false;
                let mut step = if res == 0 { 1 } else { 2 };
                loop {
                    if blocked {
                        break;
                    }
                    let x = i + (step - 1) * dir[d][0];
                    let y = j + (step - 1) * dir[d][1];
                    if x.min(y) < 0 || x.max(y) > 7 {
                        break;
                    }
                    let (x, y) = (x as usize, y as usize);
                    let mut can_stop = true;
                    for b_pp in b.iter() {
                        can_stop &= b_pp[x][y] >= 0 && b_pp[x][y] < step;
                        blocked |= (b_pp[x][y] < 0 && -b_pp[x][y] <= step) || (b_pp[x][y] == step);
                    }
                    if can_stop {
                        b[p][x][y] = -step;
                        res += _count_combinations(pieces, pos, p + 1, dir, b);
                    }
                    b[p][x][y] = step;
                    step += 1;
                }
                for b_p in b[p].iter_mut() {
                    for b_p_x in b_p.iter_mut() {
                        *b_p_x = 0;
                    }
                }
            }
            res
        }

        _count_combinations(&pieces, &positions, 0, &dir, &mut b)
    }
}

#[test]
fn test() {
    let pieces = vec!["rook".to_string()];
    let positions = vec![vec![1, 1]];
    let res = 15;
    assert_eq!(Solution::count_combinations(pieces, positions), res);
    let pieces = vec!["queen".to_string()];
    let positions = vec![vec![1, 1]];
    let res = 22;
    assert_eq!(Solution::count_combinations(pieces, positions), res);
    let pieces = vec!["bishop".to_string()];
    let positions = vec![vec![4, 3]];
    let res = 12;
    assert_eq!(Solution::count_combinations(pieces, positions), res);
}
