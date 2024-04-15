#![allow(dead_code)]

// 3117. Minimum Sum of Values by Dividing Array
// https://leetcode.com/problems/minimum-sum-of-values-by-dividing-array/
// https://leetcode.cn/problems/minimum-sum-of-values-by-dividing-array/
//
// Hard
//
// You are given two arrays nums and andValues of length n and m respectively.
//
// The value of an array is equal to the last element of that array.
//
// You have to divide nums into m disjoint contiguous subarrays such that for the ith subarray [li, ri],
// the bitwise AND of the subarray elements is equal to andValues[i],
// in other words, nums[li] & nums[li + 1] & ... & nums[ri] == andValues[i] for all 1 <= i <= m,
// where & represents the bitwise AND operator.
//
// Return the minimum possible sum of the values of the m subarrays nums is divided into.
// If it is not possible to divide nums into m subarrays satisfying these conditions, return -1.
//
// Example 1:
//
// Input: nums = [1,4,3,3,2], andValues = [0,3,3,2]
//
// Output: 12
//
// Explanation:
//
// The only possible way to divide nums is:
//
//     [1,4] as 1 & 4 == 0.
//     [3] as the bitwise AND of a single element subarray is that element itself.
//     [3] as the bitwise AND of a single element subarray is that element itself.
//     [2] as the bitwise AND of a single element subarray is that element itself.
//
// The sum of the values for these subarrays is 4 + 3 + 3 + 2 = 12.
//
// Example 2:
//
// Input: nums = [2,3,5,7,7,7,5], andValues = [0,7,5]
//
// Output: 17
//
// Explanation:
//
// There are three ways to divide nums:
//
//     [[2,3,5],[7,7,7],[5]] with the sum of the values 5 + 7 + 5 == 17.
//     [[2,3,5,7],[7,7],[5]] with the sum of the values 7 + 7 + 5 == 19.
//     [[2,3,5,7,7],[7],[5]] with the sum of the values 7 + 7 + 5 == 19.
//
// The minimum possible sum of the values is 17.
//
// Example 3:
//
// Input: nums = [1,2,3,4], andValues = [2]
//
// Output: -1
//
// Explanation:
//
// The bitwise AND of the entire array nums is 0. As there is no possible way
// to divide nums into a single subarray to have the bitwise AND of elements 2, return -1.
//
// Constraints:
//
//     1 <= n == nums.length <= 10^4
//     1 <= m == andValues.length <= min(n, 10)
//     1 <= nums[i] < 10^5
//     0 <= andValues[j] < 10^5
//

struct Solution;

impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let mut dp = vec![vec![std::collections::HashMap::new(); 11]; nums.len() + 2];
        let n = Solution::f(&mut dp, &nums, &and_values, 0, 0, (1 << 19) - 1);
        if n == 1_000_000_000 {
            -1
        } else {
            n
        }
    }

    fn f(
        dp: &mut Vec<Vec<std::collections::HashMap<i32, i32>>>,
        nums: &Vec<i32>,
        and_values: &Vec<i32>,
        i: usize,
        array_no: usize,
        ans: i32,
    ) -> i32 {
        if i >= nums.len() {
            if array_no == and_values.len() {
                return 0;
            } else {
                return 1_000_000_000;
            }
        }

        if array_no == and_values.len() {
            return 1_000_000_000;
        }

        if dp[i][array_no].contains_key(&ans) {
            return dp[i][array_no][&ans];
        }

        let mut r1 = 1_000_000_000;
        if ans & nums[i] == and_values[array_no] {
            r1 = nums[i] + Solution::f(dp, nums, and_values, i + 1, array_no + 1, (1 << 19) - 1);
        }
        let r2 = Solution::f(dp, nums, and_values, i + 1, array_no, ans & nums[i]);
        dp[i][array_no].insert(ans, r1.min(r2));
        r1.min(r2)
    }
}

#[test]
fn test() {
    let nums = vec![1, 4, 3, 3, 2];
    let and_values = vec![0, 3, 3, 2];
    let output = 12;
    assert_eq!(Solution::minimum_value_sum(nums, and_values), output);

    let nums = vec![2, 3, 5, 7, 7, 7, 5];
    let and_values = vec![0, 7, 5];
    let output = 17;
    assert_eq!(Solution::minimum_value_sum(nums, and_values), output);

    let nums = vec![1, 2, 3, 4];
    let and_values = vec![2];
    let output = -1;
    assert_eq!(Solution::minimum_value_sum(nums, and_values), output);
}
