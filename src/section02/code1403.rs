#![allow(dead_code)]

// 1403. Minimum Subsequence in Non-Increasing Order
// https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order/
// https://leetcode.cn/problems/minimum-subsequence-in-non-increasing-order/
//
// Easy
//
// Given the array nums, obtain a subsequence of the array whose sum of elements is strictly greater
// than the sum of the non included elements in such subsequence.
//
// If there are multiple solutions, return the subsequence with minimum size and if there still exist multiple solutions,
// return the subsequence with the maximum total sum of all its elements. A subsequence of an array
// can be obtained by erasing some (possibly zero) elements from the array.
//
// Note that the solution with the given constraints is guaranteed to be unique. Also return the answer sorted in non-increasing order.
//
// Example 1:
//
// Input: nums = [4,3,10,9,8]
// Output: [10,9]
// Explanation: The subsequences [10,9] and [10,8] are minimal such that the sum of their elements is strictly greater
// than the sum of elements not included. However, the subsequence [10,9] has the maximum total sum of its elements.
//
// Example 2:
//
// Input: nums = [4,4,7,6,7]
// Output: [7,7,6]
// Explanation: The subsequence [7,7] has the sum of its elements equal to 14 which is not strictly greater than
// the sum of elements not included (14 = 4 + 4 + 6). Therefore, the subsequence [7,6,7] is the minimal satisfying the conditions.
// Note the subsequence has to be returned in non-decreasing order.
//
// Constraints:
//
// -    1 <= nums.length <= 500
// -    1 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut sum = nums.iter().sum::<i32>();
        let mut res = Vec::new();
        while let Some(&x) = nums.last() {
            res.push(x);
            sum -= x;
            nums.pop();
            if res.iter().sum::<i32>() > sum {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![(vec![4, 3, 10, 9, 8], vec![10, 9]), (vec![4, 4, 7, 6, 7], vec![7, 7, 6])];
    for (nums, want) in cases {
        assert_eq!(Solution::min_subsequence(nums), want);
    }
}
