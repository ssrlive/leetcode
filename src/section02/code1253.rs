#![allow(dead_code)]

// 1253. Reconstruct a 2-Row Binary Matrix
// https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/
// https://leetcode.cn/problems/reconstruct-a-2-row-binary-matrix/
//
// Medium
//
// Given the following details of a matrix with n columns and 2 rows :
//
//     The matrix is a binary matrix, which means each element in the matrix can be 0 or 1.
//     The sum of elements of the 0-th(upper) row is given as upper.
//     The sum of elements of the 1-st(lower) row is given as lower.
//     The sum of elements in the i-th column(0-indexed) is colsum[i], where colsum is given as an integer array with length n.
//
// Your task is to reconstruct the matrix with upper, lower and colsum.
//
// Return it as a 2-D integer array.
//
// If there are more than one valid solution, any of them will be accepted.
//
// If no valid solution exists, return an empty 2-D array.
//
// Example 1:
//
// Input: upper = 2, lower = 1, colsum = [1,1,1]
// Output: [[1,1,0],[0,0,1]]
// Explanation: [[1,0,1],[0,1,0]], and [[0,1,1],[1,0,0]] are also correct answers.
//
// Example 2:
//
// Input: upper = 2, lower = 3, colsum = [2,2,1,1]
// Output: []
//
// Example 3:
//
// Input: upper = 5, lower = 5, colsum = [2,1,2,0,1,0,1,2,0,1]
// Output: [[1,1,1,0,1,0,0,1,0,0],[1,0,1,0,0,0,1,1,0,1]]
//
// Constraints:
//
// -    1 <= colsum.length <= 10^5
// -    0 <= upper, lower <= colsum.length
// -    0 <= colsum[i] <= 2
//

struct Solution;

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut upper, mut lower) = (upper, lower);
        let mut ret = vec![vec![0; colsum.len()]; 2];

        for (i, &item) in colsum.iter().enumerate() {
            if colsum[i] == 0 {
                continue;
            }
            if item == 2 {
                ret[0][i] = 1;
                ret[1][i] = 1;
                upper -= 1;
                lower -= 1;
                continue;
            }

            if upper >= lower {
                ret[0][i] = 1;
                upper -= 1;
                continue;
            }
            ret[1][i] = 1;
            lower -= 1;
        }

        if upper != 0 || lower != 0 { vec![] } else { ret }
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, 1, vec![1, 1, 1], vec![vec![0, 0, 1], vec![1, 1, 0]]),
        (2, 3, vec![2, 2, 1, 1], vec![]),
        (
            5,
            5,
            vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1],
            vec![vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1], vec![1, 1, 1, 0, 0, 0, 1, 1, 0, 0]],
        ),
    ];
    for (upper, lower, colsum, expect) in cases {
        let mut result = Solution::reconstruct_matrix(upper, lower, colsum);
        result.sort();
        assert_eq!(result, expect);
    }
}
