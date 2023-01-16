#![allow(dead_code)]

// 1186. Maximum Subarray Sum with One Deletion
// https://leetcode.com/problems/maximum-subarray-sum-with-one-deletion/
// https://leetcode.cn/problems/maximum-subarray-sum-with-one-deletion/
//
// Given an array of integers, return the maximum sum for a non-empty subarray (contiguous elements) with at most
// one element deletion. In other words, you want to choose a subarray and optionally delete one element from it
// so that there is still at least one element left and the sum of the remaining elements is maximum possible.
//
// Note that the subarray needs to be non-empty after deleting one element.
//
// Example 1:
//
// Input: arr = [1,-2,0,3]
// Output: 4
// Explanation: Because we can choose [1, -2, 0, 3] and drop -2, thus the subarray [1, 0, 3] becomes the maximum value.
//
// Example 2:
//
// Input: arr = [1,-2,-2,3]
// Output: 3
// Explanation: We just choose [3] and it's the maximum sum.
//
// Example 3:
//
// Input: arr = [-1,-1,-1,-1]
// Output: -1
// Explanation: The final subarray needs to be non-empty. You can't choose [-1] and delete -1 from it,
// then get an empty subarray to make the sum equals to 0.
//
// Constraints:
//
// - 1 <= arr.length <= 10^5
// - -10^4 <= arr[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut cur_max = arr[0];
        let mut overall_max = arr[0];
        let mut f = vec![0; n];
        let mut b = vec![0; n];
        f[0] = arr[0];
        for i in 1..n {
            cur_max = std::cmp::max(arr[i], cur_max + arr[i]);
            overall_max = std::cmp::max(overall_max, cur_max);
            f[i] = cur_max;
        }
        b[n - 1] = arr[n - 1];
        overall_max = b[n - 1];
        cur_max = overall_max;
        for i in (0..n - 1).rev() {
            cur_max = std::cmp::max(arr[i], cur_max + arr[i]);
            overall_max = std::cmp::max(overall_max, cur_max);
            b[i] = cur_max;
        }
        let mut res = overall_max;
        for i in 1..n - 1 {
            res = std::cmp::max(res, f[i - 1] + b[i + 1]);
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![1, -2, 0, 3];
    let res = 4;
    assert_eq!(Solution::maximum_sum(arr), res);
    let arr = vec![1, -2, -2, 3];
    let res = 3;
    assert_eq!(Solution::maximum_sum(arr), res);
    let arr = vec![-1, -1, -1, -1];
    let res = -1;
    assert_eq!(Solution::maximum_sum(arr), res);
    let arr = vec![8, -1, 6, -7, -4, 5, -4, 7, -6];
    let res = 17;
    assert_eq!(Solution::maximum_sum(arr), res);
}
