#![allow(dead_code)]

// 3469. Find Minimum Cost to Remove Array Elements
// https://leetcode.com/problems/find-minimum-cost-to-remove-array-elements/
// https://leetcode.cn/problems/find-minimum-cost-to-remove-array-elements/
//
// Medium
//
// You are given an integer array nums. Your task is to remove all elements from the array by performing one of the following operations at each step until nums is empty:
//
// Choose any two elements from the first three elements of nums and remove them. The cost of this operation is the maximum of the two elements removed.
// If fewer than three elements remain in nums, remove all the remaining elements in a single operation. The cost of this operation is the maximum of the remaining elements.
// Return the minimum cost required to remove all the elements.
//
// Example 1:
//
// Input: nums = [6,2,8,4]
//
// Output: 12
//
// Explanation:
//
// Initially, nums = [6, 2, 8, 4].
//
// In the first operation, remove nums[0] = 6 and nums[2] = 8 with a cost of max(6, 8) = 8. Now, nums = [2, 4].
// In the second operation, remove the remaining elements with a cost of max(2, 4) = 4.
// The cost to remove all elements is 8 + 4 = 12. This is the minimum cost to remove all elements in nums. Hence, the output is 12.
//
// Example 2:
//
// Input: nums = [2,1,3,3]
//
// Output: 5
//
// Explanation:
//
// Initially, nums = [2, 1, 3, 3].
//
// In the first operation, remove nums[0] = 2 and nums[1] = 1 with a cost of max(2, 1) = 2. Now, nums = [3, 3].
// In the second operation remove the remaining elements with a cost of max(3, 3) = 3.
// The cost to remove all elements is 2 + 3 = 5. This is the minimum cost to remove all elements in nums. Hence, the output is 5.
//
// Constraints:
//
// 1 <= nums.length <= 1000
// 1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>) -> i32 {
        fn cal(ext: i32, ind: i32, nums: &[i32], dp: &mut Vec<Vec<i32>>) -> i32 {
            if ind == nums.len() as i32 {
                return nums[ext as usize];
            }

            if ind == nums.len() as i32 - 1 {
                return nums[ext as usize].max(nums[ind as usize]);
            }

            if dp[ind as usize][ext as usize] != -1 {
                return dp[ind as usize][ext as usize];
            }

            let f = nums[ind as usize].max(nums[ind as usize + 1]) + cal(ext, ind + 2, nums, dp);
            let s = nums[ext as usize].max(nums[ind as usize + 1]) + cal(ind, ind + 2, nums, dp);
            let t = nums[ext as usize].max(nums[ind as usize]) + cal(ind + 1, ind + 2, nums, dp);

            dp[ind as usize][ext as usize] = f.min(s).min(t);
            dp[ind as usize][ext as usize]
        }

        let n = nums.len();
        let mut dp = vec![vec![-1; n]; n];
        cal(0, 1, &nums, &mut dp)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_cost(vec![6, 2, 8, 4]), 12);
    assert_eq!(Solution::min_cost(vec![2, 1, 3, 3]), 5);
}
