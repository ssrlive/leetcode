#![allow(dead_code)]

// 3139. Minimum Cost to Equalize Array
// https://leetcode.com/problems/minimum-cost-to-equalize-array/
// https://leetcode.cn/problems/minimum-cost-to-equalize-array/
//
// Hard
//
// You are given an integer array nums and two integers cost1 and cost2.
// You are allowed to perform either of the following operations any number of times:
//
// - Choose an index i from nums and increase nums[i] by 1 for a cost of cost1.
// - Choose two different indices i, j, from nums and increase nums[i] and nums[j] by 1 for a cost of cost2.
//
// Return the minimum cost required to make all elements in the array equal.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [4,1], cost1 = 5, cost2 = 2
//
// Output: 15
//
// Explanation:
//
// The following operations can be performed to make the values equal:
//
// Increase nums[1] by 1 for a cost of 5. nums becomes [4,2].
// Increase nums[1] by 1 for a cost of 5. nums becomes [4,3].
// Increase nums[1] by 1 for a cost of 5. nums becomes [4,4].
// The total cost is 15.
//
// Example 2:
//
// Input: nums = [2,3,3,3,5], cost1 = 2, cost2 = 1
//
// Output: 6
//
// Explanation:
//
// The following operations can be performed to make the values equal:
//
// Increase nums[0] and nums[1] by 1 for a cost of 1. nums becomes [3,4,3,3,5].
// Increase nums[0] and nums[2] by 1 for a cost of 1. nums becomes [4,4,4,3,5].
// Increase nums[0] and nums[3] by 1 for a cost of 1. nums becomes [5,4,4,4,5].
// Increase nums[1] and nums[2] by 1 for a cost of 1. nums becomes [5,5,5,4,5].
// Increase nums[3] by 1 for a cost of 2. nums becomes [5,5,5,5,5].
// The total cost is 6.
//
// Example 3:
//
// Input: nums = [3,5,3], cost1 = 1, cost2 = 3
//
// Output: 4
//
// Explanation:
//
// The following operations can be performed to make the values equal:
//
// Increase nums[0] by 1 for a cost of 1. nums becomes [4,5,3].
// Increase nums[0] by 1 for a cost of 1. nums becomes [5,5,3].
// Increase nums[2] by 1 for a cost of 1. nums becomes [5,5,4].
// Increase nums[2] by 1 for a cost of 1. nums becomes [5,5,5].
// The total cost is 4.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^6
// 1 <= cost1 <= 10^6
// 1 <= cost2 <= 10^6
//

struct Solution;

impl Solution {
    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        let n = nums.len();
        let mut maxi = 0;
        let mut tot = 0;
        let mut mini = 1_000_000_000_i64;
        for &nums_i in nums.iter().take(n) {
            maxi = maxi.max(nums_i);
            mini = mini.min(nums_i);
            tot += nums_i;
        }

        let mut ans = 100_000_000_000_000_000;

        for eq in (maxi..=maxi * 2).rev() {
            let mut rem = eq * n as i64 - tot;
            let minadd = eq - mini;
            let mut curcost = 0;

            if minadd * 2 >= rem {
                let extra = minadd - (rem - minadd);
                curcost += extra * cost1 as i64;
                rem -= extra;
            }

            if rem % 2 == 1 {
                rem -= 1;
                curcost += cost1 as i64;
            }

            curcost += (cost1 as i64 * rem).min(cost2 as i64 * (rem / 2));
            ans = ans.min(curcost);
        }

        (ans % 1_000_000_007) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_cost_to_equalize_array(vec![4, 1], 5, 2), 15);
    assert_eq!(Solution::min_cost_to_equalize_array(vec![2, 3, 3, 3, 5], 2, 1), 6);
    assert_eq!(Solution::min_cost_to_equalize_array(vec![3, 5, 3], 1, 3), 4);
}
