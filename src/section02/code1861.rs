#![allow(dead_code)]

/*

// 1861. Rotating the Box
// https://leetcode.com/problems/rotating-the-box/
// https://leetcode.cn/problems/rotating-the-box/
//
// Medium
//
// You are given an m x n matrix of characters box representing a side-view of a box. Each cell of the box is one of the following:

    A stone '#'
    A stationary obstacle '*'
    Empty '.'

The box is rotated 90 degrees clockwise, causing some of the stones to fall due to gravity. Each stone falls down until it lands on an obstacle, another stone, or the bottom of the box. Gravity does not affect the obstacles' positions, and the inertia from the box's rotation does not affect the stones' horizontal positions.

It is guaranteed that each stone in box rests on an obstacle, another stone, or the bottom of the box.

Return an n x m matrix representing the box after the rotation described above.

Example 1:

Input: box = [["#",".","#"]]
Output: [["."],
         ["#"],
         ["#"]]

Example 2:

Input: box = [["#",".","*","."],
              ["#","#","*","."]]
Output: [["#","."],
         ["#","#"],
         ["*","*"],
         [".","."]]

Example 3:

Input: box = [["#","#","*",".","*","."],
              ["#","#","#","*",".","."],
              ["#","#","#",".","#","."]]
Output: [[".","#","#"],
         [".","#","#"],
         ["#","#","*"],
         ["#","*","."],
         ["#",".","*"],
         ["#",".","."]]

Constraints:

    m == box.length
    n == box[i].length
    1 <= m, n <= 500
    box[i][j] is either '#', '*', or '.'.
*/

struct Solution;

impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut rotated = vec![vec!['.'; b.len()]; b[0].len()];
        for i in 0..b.len() {
            let mut count = 0;
            for j in 0..b[0].len() {
                if b[i][j] == '#' {
                    count += 1;
                } else if b[i][j] == '*' {
                    rotated[j][b.len() - 1 - i] = '*';
                }
                if b[i][j] == '*' || j == b[0].len() - 1 {
                    for k in 0..count {
                        rotated[j - k - if b[i][j] == '*' { 1 } else { 0 }][b.len() - 1 - i] = '#';
                    }
                    count = 0;
                }
            }
        }
        rotated
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec!['#', '.', '#']], vec![vec!['.'], vec!['#'], vec!['#']]),
        (
            vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']],
            vec![vec!['#', '.'], vec!['#', '#'], vec!['*', '*'], vec!['.', '.']],
        ),
        (
            vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.'],
            ],
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.'],
            ],
        ),
    ];
    for (box_, expect) in cases {
        assert_eq!(Solution::rotate_the_box(box_), expect);
    }
}
