#![allow(dead_code)]

// 85. Maximal Rectangle
// https://leetcode.com/problems/maximal-rectangle/
// https://leetcode.cn/problems/maximal-rectangle/
//
// Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
//
// Example 1:
//
// Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
// Output: 6
// Explanation: The maximal rectangle is shown in the above picture.
//
// Example 2:
//
// Input: matrix = [["0"]]
// Output: 0
//
// Example 3:
//
// Input: matrix = [["1"]]
// Output: 1
//
// Constraints:
//
// rows == matrix.length
// cols == matrix[i].length
// 1 <= row, cols <= 200
// matrix[i][j] is '0' or '1'.
//

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        pub fn max_in_histogram(hist: &[i32]) -> i32 {
            let mut res: i32 = 0;
            let mut stack: Vec<usize> = vec![];

            for (i, val) in hist.iter().enumerate() {
                while !stack.is_empty() && hist[*stack.iter().last().unwrap()] > *val {
                    let j = stack.pop().unwrap();
                    let width = (i - stack[stack.len() - 1] - 1) as i32;
                    res = res.max(hist[j] * width);
                }
                stack.push(i);
            }

            res
        }

        if matrix.is_empty() {
            return 0;
        }

        let mut res = 0;
        let mut histogram: Vec<i32> = vec![0; matrix[0].len() + 2];

        for row in matrix.iter() {
            for (i, val) in row.iter().enumerate() {
                if *val == '0' {
                    histogram[i + 1] = 0;
                } else {
                    histogram[i + 1] += 1;
                }
            }
            res = res.max(max_in_histogram(&histogram));
        }

        res
    }
}

#[test]
fn test() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(Solution::maximal_rectangle(matrix), 6);

    let matrix = vec![vec!['0']];
    assert_eq!(Solution::maximal_rectangle(matrix), 0);

    let matrix = vec![vec!['1']];
    assert_eq!(Solution::maximal_rectangle(matrix), 1);
}
