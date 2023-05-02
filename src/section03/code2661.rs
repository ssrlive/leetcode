#![allow(dead_code)]

/*
// 2661. First Completely Painted Row or Column
// https://leetcode.com/problems/first-completely-painted-row-or-column/
// https://leetcode.cn/problems/first-completely-painted-row-or-column/
//
// Medium
//
// You are given a 0-indexed integer array arr, and an m x n integer matrix mat.
// arr and mat both contain all the integers in the range [1, m * n].

Go through each index i in arr starting from index 0 and paint the cell in mat containing the integer arr[i].

Return the smallest index i at which either a row or a column will be completely painted in mat.

Example 1:
image explanation for example 1

Input: arr = [1,3,4,2], mat = [[1,4],[2,3]]
Output: 2
Explanation: The moves are shown in order, and both the first row and second column of the matrix become fully painted at arr[2].

Example 2:
image explanation for example 2

Input: arr = [2,8,7,4,1,3,5,6,9], mat = [[3,2,5],[1,4,6],[8,7,9]]
Output: 3
Explanation: The second column becomes fully painted at arr[3].

Constraints:

    m == mat.length
    n = mat[i].length
    arr.length == m * n
    1 <= m, n <= 10^5
    1 <= m * n <= 10^5
    1 <= arr[i], mat[r][c] <= m * n
    All the integers of arr are unique.
    All the integers of mat are unique.
*/

struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut ans = i32::MAX;
        let mut map = HashMap::new();
        for (i, &num) in arr.iter().enumerate() {
            map.insert(num, i);
        }

        for item in mat.iter() {
            let mut idx = 0;
            for num in item.iter() {
                idx = idx.max(map[num]);
            }
            ans = ans.min(idx as i32);
        }

        for i in 0..mat[0].len() {
            let mut idx = 0;
            for j in 0..mat.len() {
                idx = idx.max(map[&mat[j][i]]);
            }
            ans = ans.min(idx as i32);
        }

        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]], 2),
        (
            vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
            vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]],
            3,
        ),
    ];
    for (arr, mat, expect) in cases {
        assert_eq!(Solution::first_complete_index(arr, mat), expect);
    }
}
