#![allow(dead_code)]

/*

// 1695. Maximum Erasure Value
// https://leetcode.com/problems/maximum-erasure-value/
// https://leetcode.cn/problems/maximum-erasure-value/
//
// Medium
//
// You are given an array of positive integers nums and want to erase a subarray containing unique elements. The score you get by erasing the subarray is equal to the sum of its elements.

Return the maximum score you can get by erasing exactly one subarray.

An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).

Example 1:

Input: nums = [4,2,4,5,6]
Output: 17
Explanation: The optimal subarray here is [2,4,5,6].

Example 2:

Input: nums = [5,2,1,2,5,2,1,2,5]
Output: 8
Explanation: The optimal subarray here is [5,2,1] or [1,2,5].

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        let mut set = std::collections::HashSet::new();
        let mut i = 0;
        for j in 0..nums.len() {
            while set.contains(&nums[j]) {
                set.remove(&nums[i]);
                sum -= nums[i];
                i += 1;
            }
            set.insert(nums[j]);
            sum += nums[j];
            ans = ans.max(sum);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(vec![4, 2, 4, 5, 6], 17), (vec![5, 2, 1, 2, 5, 2, 1, 2, 5], 8)];
    for (nums, expected) in cases {
        assert_eq!(Solution::maximum_unique_subarray(nums), expected);
    }
}
