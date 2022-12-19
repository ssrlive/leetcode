#![allow(dead_code)]

// 363. Max Sum of Rectangle No Larger Than K
// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
// https://leetcode.cn/problems/max-sum-of-rectangle-no-larger-than-k/
//
// Given an m x n matrix matrix and an integer k, return the max sum of a rectangle in the matrix such that its sum is no larger than k.
//
// It is guaranteed that there will be a rectangle with a sum no larger than k.
//
// Example 1:
//
// Input: matrix = [[1,0,1],[0,-2,3]], k = 2
// Output: 2
// Explanation: Because the sum of the blue rectangle [[0, 1], [-2, 3]] is 2,
// and 2 is the max number no larger than k (k = 2).
//
// Example 2:
//
// Input: matrix = [[2,2,-1]], k = 3
// Output: 3
//
// Constraints:
//
// - m == matrix.length
// - n == matrix[i].length
// - 1 <= m, n <= 100
// - -100 <= matrix[i][j] <= 100
// - -10^5 <= k <= 10^5
//
// Follow up: What if the number of rows is much larger than the number of columns?
//

struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut max = std::i32::MIN;
        for i in 0..m {
            let mut sums = vec![0; n];
            for item in matrix.iter().take(m).skip(i) {
                for (c, v) in sums.iter_mut().enumerate() {
                    *v += item[c];
                }
                let mut set = std::collections::BTreeSet::new();
                set.insert(0);
                let mut sum = 0;
                for s in sums.iter() {
                    sum += s;
                    if let Some(&x) = set.range(sum - k..).next() {
                        max = max.max(sum - x);
                    }
                    set.insert(sum);
                }
            }
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
    assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3), 3);
}
