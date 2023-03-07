#![allow(dead_code)]

/*

// 2326. Spiral Matrix IV
// https://leetcode.com/problems/spiral-matrix-iv/
// https://leetcode.cn/problems/spiral-matrix-iv/
//
// Medium
//
// You are given two integers m and n, which represent the dimensions of a matrix.

You are also given the head of a linked list of integers.

Generate an m x n matrix that contains the integers in the linked list presented in spiral order (clockwise), starting from the top-left of the matrix. If there are remaining empty spaces, fill them with -1.

Return the generated matrix.

Example 1:

Input: m = 3, n = 5, head = [3,0,2,6,8,1,7,9,4,2,5,5,0]
Output: [[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]
Explanation: The diagram above shows how the values are printed in the matrix.
Note that the remaining spaces in the matrix are filled with -1.

Example 2:

Input: m = 1, n = 4, head = [0,1,2]
Output: [[0,1,2,-1]]
Explanation: The diagram above shows how the values are printed from left to right in the matrix.
The last space in the matrix is set to -1.

Constraints:

    1 <= m, n <= 10^5
    1 <= m * n <= 10^5
    The number of nodes in the list is in the range [1, m * n].
    0 <= Node.val <= 1000
*/

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        enum Direction {
            Up,
            Down,
            Left,
            Right,
        }

        let (mut n, mut m) = (n, m);
        let (row, col) = (m, n);
        let mut result: Vec<Vec<i32>> = vec![vec![0; col as usize]; row as usize];

        let mut y: usize = 0;
        let mut x: usize = 0;
        let mut direction: Direction = Direction::Right;

        let mut pointer = &head;
        for _count in 0..row * col {
            let mut val: i32 = -1;
            if let Some(node) = pointer {
                val = node.val;
                pointer = &node.next;
            }
            result[y][x] = val;
            match direction {
                Direction::Right => {
                    if x as i32 == n - 1 {
                        direction = Direction::Down;
                        y += 1;
                    } else {
                        x += 1;
                    }
                }
                Direction::Down => {
                    if y as i32 == m - 1 {
                        direction = Direction::Left;
                        x -= 1;
                        m -= 1;
                    } else {
                        y += 1;
                    }
                }
                Direction::Left => {
                    if x as i32 == col - n {
                        direction = Direction::Up;
                        y -= 1;
                        n -= 1;
                    } else {
                        x -= 1;
                    }
                }
                Direction::Up => {
                    if y as i32 == row - m {
                        direction = Direction::Right;
                        x += 1;
                    } else {
                        y -= 1;
                    }
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
            3,
            5,
            vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0],
            vec![vec![3, 0, 2, 6, 8], vec![5, 0, -1, -1, 1], vec![5, 2, 4, 9, 7]],
        ),
        (1, 4, vec![0, 1, 2], vec![vec![0, 1, 2, -1]]),
    ];
    for (m, n, head, expected) in cases {
        let head = ListNode::from_vec(&head);
        assert_eq!(Solution::spiral_matrix(m, n, head), expected);
    }
}
