#![allow(dead_code)]

// 2536. Increment Submatrices by One
// https://leetcode.com/problems/increment-submatrices-by-one/
// https://leetcode.cn/problems/increment-submatrices-by-one/
//
// You are given a positive integer n, indicating that we initially have an n x n 0-indexed integer matrix mat filled with zeroes.
//
// You are also given a 2D integer array query. For each query[i] = [row1i, col1i, row2i, col2i], you should do the following operation:
//
// Add 1 to every element in the submatrix with the top left corner (row1i, col1i) and the bottom right corner (row2i, col2i).
// That is, add 1 to mat[x][y] for for all row1i <= x <= row2i and col1i <= y <= col2i.
//
// Return the matrix mat after performing every query.
//
// Example 1:
//
// Input: n = 3, queries = [[1,1,2,2],[0,0,1,1]]
// Output: [[1,1,0],[1,2,1],[0,1,1]]
// Explanation: The diagram above shows the initial matrix, the matrix after the first query, and the matrix after the second query.
// - In the first query, we add 1 to every element in the submatrix with the top left corner (1, 1) and bottom right corner (2, 2).
// - In the second query, we add 1 to every element in the submatrix with the top left corner (0, 0) and bottom right corner (1, 1).
//
// Example 2:
//
// Input: n = 2, queries = [[0,0,1,1]]
// Output: [[1,1],[1,1]]
// Explanation: The diagram above shows the initial matrix and the matrix after the first query.
// - In the first query we add 1 to every element in the matrix.
//
// Constraints:
//
// - 1 <= n <= 500
// - 1 <= queries.length <= 10^4
// - 0 <= row1i <= row2i < n
// - 0 <= col1i <= col2i < n
//

struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut data = vec![vec![0; n + 1]; n];

        for q in queries {
            let (x, y, u, v) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            for item in data.iter_mut().skip(x).take(u + 1) {
                item[y] += 1;
                item[v + 1] -= 1;
            }
        }

        let mut ret = vec![];
        for item in data.iter() {
            let mut v = vec![];
            let mut sum = 0;
            for &item2 in item.iter().take(n) {
                sum += item2;
                v.push(sum);
            }
            ret.push(v);
        }

        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            3,
            vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]],
            vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]],
        ),
        (2, vec![vec![0, 0, 1, 1]], vec![vec![1, 1], vec![1, 1]]),
    ];
    for (n, queries, expect) in cases {
        assert_eq!(Solution::range_add_queries(n, queries), expect);
    }
}
