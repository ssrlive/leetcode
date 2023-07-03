#![allow(dead_code)]

// 59. Spiral Matrix II
// https://leetcode.com/problems/spiral-matrix-ii/
// https://leetcode.cn/problems/spiral-matrix-ii/
//
// Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.
//
// Example 1:
//
// Input: n = 3
// Output: [[1,2,3],[8,9,4],[7,6,5]]
//
// Example 2:
//
// Input: n = 1
// Output: [[1]]
//
// Constraints:
//
// - 1 <= n <= 20
//

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let un = n as usize;
        let mut result = vec![vec![0; un]; un];
        let mut i = 0i32;
        let mut j = 0i32;
        let mut a = 0;
        let p = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut v = 1;
        while v <= n * n {
            result[i as usize][j as usize] = v;
            let (x, y) = p[a % 4];
            let ni = i + x;
            let nj = j + y;
            if ni < 0 || n <= ni || nj < 0 || n <= nj || result[ni as usize][nj as usize] != 0 {
                a += 1;
                let (x, y) = p[a % 4];
                i += x;
                j += y;
            } else {
                i = ni;
                j = nj;
            }
            v += 1;
        }

        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (1, vec![vec![1]]),
        (2, vec![vec![1, 2], vec![4, 3]]),
        (3, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]),
        (
            4,
            vec![vec![1, 2, 3, 4], vec![12, 13, 14, 5], vec![11, 16, 15, 6], vec![10, 9, 8, 7]],
        ),
    ];
    for (n, expected) in cases {
        assert_eq!(Solution::generate_matrix(n), expected);
    }
}
