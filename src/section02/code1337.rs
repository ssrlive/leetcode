#![allow(dead_code)]

// 1337. The K Weakest Rows in a Matrix
// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
// https://leetcode.cn/problems/the-k-weakest-rows-in-a-matrix/
//
// Easy
//
// You are given an m x n binary matrix mat of 1's (representing soldiers) and 0's (representing civilians).
// The soldiers are positioned in front of the civilians. That is, all the 1's will appear to the left of all the 0's in each row.
//
// A row i is weaker than a row j if one of the following is true:
//
//     The number of soldiers in row i is less than the number of soldiers in row j.
//     Both rows have the same number of soldiers and i < j.
//
// Return the indices of the k weakest rows in the matrix ordered from weakest to strongest.
//
// Example 1:
//
// Input: mat =
// [[1,1,0,0,0],
//  [1,1,1,1,0],
//  [1,0,0,0,0],
//  [1,1,0,0,0],
//  [1,1,1,1,1]],
// k = 3
// Output: [2,0,3]
// Explanation:
// The number of soldiers in each row is:
// - Row 0: 2
// - Row 1: 4
// - Row 2: 1
// - Row 3: 2
// - Row 4: 5
// The rows ordered from weakest to strongest are [2,0,3,1,4].
//
// Example 2:
//
// Input: mat =
// [[1,0,0,0],
//  [1,1,1,1],
//  [1,0,0,0],
//  [1,0,0,0]],
// k = 2
// Output: [0,2]
// Explanation:
// The number of soldiers in each row is:
// - Row 0: 1
// - Row 1: 4
// - Row 2: 1
// - Row 3: 1
// The rows ordered from weakest to strongest are [0,2,3,1].
//
// Constraints:
//
// -    m == mat.length
// -    n == mat[i].length
// -    2 <= n, m <= 100
// -    1 <= k <= m
// -    matrix[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let f = |(i, row): (usize, &Vec<i32>)| (i, row.iter().filter(|&&x| x == 1).count());
        let mut rows = mat.iter().enumerate().map(f).collect::<Vec<(usize, usize)>>();
        rows.sort_by(|a, b| a.1.cmp(&b.1));
        rows.iter().map(|x| x.0 as i32).take(k as usize).collect()
    }
}

#[test]
fn test() {
    let mat = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
    ];
    let k = 3;
    let result = vec![2, 0, 3];
    assert_eq!(Solution::k_weakest_rows(mat, k), result);

    let mat = vec![vec![1, 0, 0, 0], vec![1, 1, 1, 1], vec![1, 0, 0, 0], vec![1, 0, 0, 0]];
    let k = 2;
    let result = vec![0, 2];
    assert_eq!(Solution::k_weakest_rows(mat, k), result);
}
