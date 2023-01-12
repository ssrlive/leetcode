#![allow(dead_code)]

// 1131. Maximum of Absolute Value Expression
// https://leetcode.com/problems/maximum-of-absolute-value-expression/
// https://leetcode.cn/problems/maximum-of-absolute-value-expression/
//
// Given two arrays of integers with equal lengths, return the maximum value of:
//
// |arr1[i] - arr1[j]| + |arr2[i] - arr2[j]| + |i - j|
//
// where the maximum is taken over all 0 <= i, j < arr1.length.
//
// Example 1:
//
// Input: arr1 = [1,2,3,4], arr2 = [-1,4,5,6]
// Output: 13
//
// Example 2:
//
// Input: arr1 = [1,-2,-5,0,10], arr2 = [0,-2,-1,-7,-4]
// Output: 20
//
// Constraints:
//
// - 2 <= arr1.length == arr2.length <= 40000
// - -10^6 <= arr1[i], arr2[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        fn fun(arr: &[i32], n: usize) -> i32 {
            let mut max_sum = arr[0];
            let mut min_sum = arr[0];
            for &item in arr.iter().take(n) {
                max_sum = std::cmp::max(max_sum, item);
                min_sum = std::cmp::min(min_sum, item);
            }
            max_sum - min_sum
        }

        let n = arr1.len();
        let mut sum1 = vec![0; n];
        let mut diff1 = vec![0; n];
        let mut sum2 = vec![0; n];
        let mut diff2 = vec![0; n];
        for i in 0..n {
            sum1[i] = arr1[i] + arr2[i] + i as i32;
            diff1[i] = arr1[i] - arr2[i] + i as i32;
            sum2[i] = arr1[i] + arr2[i] - i as i32;
            diff2[i] = arr1[i] - arr2[i] - i as i32;
        }
        let mut result = 0;
        result = std::cmp::max(result, fun(&sum1, n));
        result = std::cmp::max(result, fun(&diff1, n));
        result = std::cmp::max(result, fun(&sum2, n));
        result = std::cmp::max(result, fun(&diff2, n));
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], vec![-1, 4, 5, 6], 13),
        (vec![1, -2, -5, 0, 10], vec![0, -2, -1, -7, -4], 20),
    ];
    for (arr1, arr2, expected) in cases {
        assert_eq!(Solution::max_abs_val_expr(arr1, arr2), expected);
    }
}
