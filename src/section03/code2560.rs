#![allow(dead_code)]

// 2560. House Robber IV
// https://leetcode.com/problems/house-robber-iv/
// https://leetcode.cn/problems/house-robber-iv/
//
// Medium
//
// There are several consecutive houses along a street, each of which has some money inside.
// There is also a robber, who wants to steal money from the homes, but he refuses to steal from adjacent homes.
//
// The capability of the robber is the maximum amount of money he steals from one house of all the houses he robbed.
//
// You are given an integer array nums representing how much money is stashed in each house.
// More formally, the ith house from the left has nums[i] dollars.
//
// You are also given an integer k, representing the minimum number of houses the robber will steal from.
// It is always possible to steal at least k houses.
//
// Return the minimum capability of the robber out of all the possible ways to steal at least k houses.
//
// Example 1:
//
// Input: nums = [2,3,5,9], k = 2
// Output: 5
// Explanation:
// There are three ways to rob at least 2 houses:
// - Rob the houses at indices 0 and 2. Capability is max(nums[0], nums[2]) = 5.
// - Rob the houses at indices 0 and 3. Capability is max(nums[0], nums[3]) = 9.
// - Rob the houses at indices 1 and 3. Capability is max(nums[1], nums[3]) = 9.
// Therefore, we return min(5, 9, 9) = 5.
//
// Example 2:
//
// Input: nums = [2,7,9,3,1], k = 2
// Output: 2
// Explanation: There are 7 ways to rob the houses. The way which leads to minimum capability is
// to rob the house at index 0 and 4. Return max(nums[0], nums[4]) = 2.
//
// Constraints:
//
// -    1 <= nums.length <= 10^5
// -    1 <= nums[i] <= 10^9
// -    1 <= k <= (nums.length + 1)/2
//

struct Solution;

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        fn check(nums: &Vec<i32>, k: i32, val: i32) -> bool {
            let (mut i, n, mut k) = (0, nums.len(), k);
            while i < n {
                if nums[i] <= val {
                    k -= 1;
                    i += 1;
                }
                if k == 0 {
                    return true;
                }
                i += 1;
            }

            false
        }

        let (mut left, mut right) = (0, i32::MAX);

        while left < right {
            let mid = left + (right - left) / 2;
            if check(&nums, k, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

#[test]
fn test() {
    let cases = vec![(vec![2, 3, 5, 9], 2, 5), (vec![2, 7, 9, 3, 1], 2, 2)];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::min_capability(nums, k), expected);
    }
}
