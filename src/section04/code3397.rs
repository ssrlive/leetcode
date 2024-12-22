#![allow(dead_code)]

// 3397. Maximum Number of Distinct Elements After Operations
// https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/
// https://leetcode.cn/problems/maximum-number-of-distinct-elements-after-operations/
//
// Medium
//
// You are given an integer array nums and an integer k.
//
// You are allowed to perform the following operation on each element of the array at most once:
//
// Add an integer in the range [-k, k] to the element.
// Return the maximum possible number of distinct elements in nums after performing the operations.
//
// Example 1:
//
// Input: nums = [1,2,2,3,3,4], k = 2
//
// Output: 6
//
// Explanation:
//
// nums changes to [-1, 0, 1, 2, 3, 4] after performing operations on the first four elements.
//
// Example 2:
//
// Input: nums = [4,4,4,4], k = 1
//
// Output: 3
//
// Explanation:
//
// By adding -1 to nums[0] and 1 to nums[1], nums changes to [3, 5, 4, 4].
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^9
// 0 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut nums = nums;
        nums.sort();
        let mut last = i32::MIN;
        for &nums_i in &nums {
            let mn = nums_i - k;
            let mx = nums_i + k;
            if last < mn {
                last = mn;
                ans += 1;
            } else if last < mx {
                last += 1;
                ans += 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 2, 3, 3, 4];
    let k = 2;
    let output = 6;
    assert_eq!(Solution::max_distinct_elements(nums, k), output);

    let nums = vec![4, 4, 4, 4];
    let k = 1;
    let output = 3;
    assert_eq!(Solution::max_distinct_elements(nums, k), output);
}
