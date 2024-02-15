#![allow(dead_code)]

// 1388. Pizza With 3n Slices
// https://leetcode.com/problems/pizza-with-3n-slices/
// https://leetcode.cn/problems/pizza-with-3n-slices/
//
// Hard
//
// There is a pizza with 3n slices of varying size, you and your friends will take slices of pizza as follows:
//
//     You will pick any pizza slice.
//     Your friend Alice will pick the next slice in the anti-clockwise direction of your pick.
//     Your friend Bob will pick the next slice in the clockwise direction of your pick.
//     Repeat until there are no more slices of pizzas.
//
// Given an integer array slices that represent the sizes of the pizza slices in a clockwise direction,
// return the maximum possible sum of slice sizes that you can pick.
//
// Example 1:
//
// Input: slices = [1,2,3,4,5,6]
// Output: 10
// Explanation: Pick pizza slice of size 4, Alice and Bob will pick slices with size 3 and 5 respectively.
// Then Pick slices with size 6, finally Alice and Bob will pick slice of size 2 and 1 respectively. Total = 4 + 6.
//
// Example 2:
//
// Input: slices = [8,9,8,6,1,1]
// Output: 16
// Explanation: Pick pizza slice of size 8 in each turn. If you pick slice with size 9 your partners will pick slices of size 8.
//
// Constraints:
//
// -    3 * n == slices.length
// -    1 <= slices.length <= 500
// -    1 <= slices[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        fn max_sum(slices: &[i32], n: usize) -> i32 {
            let m = slices.len();
            let mut dp = vec![vec![0; n + 1]; m + 1];
            for i in 1..=m {
                for j in 1..=n {
                    if i == 1 {
                        dp[i][j] = slices[0];
                    } else {
                        dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 2][j - 1] + slices[i - 1]);
                    }
                }
            }
            dp[m][n]
        }

        let m = slices.len();
        let n = m / 3;
        let slices1 = slices[0..m - 1].to_vec();
        let slices2 = slices[1..m].to_vec();
        let val1 = max_sum(&slices1, n);
        let val2 = max_sum(&slices2, n);
        std::cmp::max(val1, val2)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 5, 6], 10),
        (vec![8, 9, 8, 6, 1, 1], 16),
        (vec![4, 1, 2, 5, 8, 3, 1, 9, 7], 21),
        (vec![3, 1, 2], 3),
        (vec![3, 1, 2, 5, 8, 3, 1, 9, 7], 20),
        (vec![3, 1, 2, 5, 8, 4, 1, 9, 7], 20),
    ];
    for (slices, expected) in cases {
        assert_eq!(Solution::max_size_slices(slices), expected);
    }
}
