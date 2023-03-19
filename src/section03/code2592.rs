#![allow(dead_code)]

/*

// 2592. Maximize Greatness of an Array
// https://leetcode.com/problems/maximize-greatness-of-an-array/
// https://leetcode.cn/problems/maximize-greatness-of-an-array/
//
// Medium
//
// You are given a 0-indexed integer array nums. You are allowed to permute nums into a new array perm of your choosing.

We define the greatness of nums be the number of indices 0 <= i < nums.length for which perm[i] > nums[i].

Return the maximum possible greatness you can achieve after permuting nums.

Example 1:

Input: nums = [1,3,5,2,1,3,1]
Output: 4
Explanation: One of the optimal rearrangements is perm = [2,5,1,3,3,1,1].
At indices = 0, 1, 3, and 4, perm[i] > nums[i]. Hence, we return 4.

Example 2:

Input: nums = [1,2,3,4]
Output: 3
Explanation: We can prove the optimal perm is [2,3,4,1].
At indices = 0, 1, and 2, perm[i] > nums[i]. Hence, we return 3.

Constraints:

    1 <= nums.length <= 10^5
    0 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn maximize_greatness(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let (mut ans, mut l) = (0, 0);
        for i in 0..n - 1 {
            let mut r = n;
            l += 1;
            while l < r {
                let mid = l + (r - l) / 2;
                if nums[mid] > nums[i] {
                    r = mid
                } else {
                    l = mid + 1
                };
            }
            if l < n && nums[l] > nums[i] {
                ans += 1
            } else {
                break;
            };
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximize_greatness(vec![1, 3, 5, 2, 1, 3, 1]), 4);
    assert_eq!(Solution::maximize_greatness(vec![1, 2, 3, 4]), 3);
    assert_eq!(Solution::maximize_greatness(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::maximize_greatness(vec![5, 4, 3, 2, 1]), 4);
}
