#![allow(dead_code)]

// 2741. Special Permutations
// https://leetcode.com/problems/special-permutations/
// https://leetcode.cn/problems/special-permutations/
//
// Medium
//
// You are given a 0-indexed integer array nums containing n distinct positive integers.
// A permutation of nums is called special if:
//
//     For all indexes 0 <= i < n - 1, either nums[i] % nums[i+1] == 0 or nums[i+1] % nums[i] == 0.
//
// Return the total number of special permutations. As the answer could be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [2,3,6]
// Output: 2
// Explanation: [3,6,2] and [2,6,3] are the two special permutations of nums.
//
// Example 2:
//
// Input: nums = [1,4,3]
// Output: 2
// Explanation: [3,1,4] and [4,1,3] are the two special permutations of nums.
//
// Constraints:
//
//     2 <= nums.length <= 14
//     1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        const MOD: i32 = (10_i32).pow(9) + 7;
        use std::collections::HashMap;
        let n: usize = nums.len();
        let mut dp: HashMap<(usize, usize), i32> = HashMap::new();
        let mut result: i32 = 0;
        for index in 0..n {
            result = (result + Self::dfs(index, &nums, &mut dp, 1 << index)) % MOD;
        }

        result
    }

    fn dfs(index_previous: usize, nums: &Vec<i32>, dp: &mut std::collections::HashMap<(usize, usize), i32>, mask: usize) -> i32 {
        const MOD: i32 = (10_i32).pow(9) + 7;
        let n = nums.len();
        if mask == ((1 << n) - 1) {
            return 1;
        }

        if dp.contains_key(&(index_previous, mask)) {
            return *dp.get(&(index_previous, mask)).unwrap();
        }

        let mut permutation_amount: i32 = 0;
        for index_target in 0..n {
            let mask_target = 1 << index_target;
            if (mask & mask_target) != 0 {
                continue;
            }

            if nums[index_previous] % nums[index_target] == 0 || nums[index_target] % nums[index_previous] == 0 {
                let mask_next = mask | mask_target;
                let temp: i32 = Self::dfs(index_target, nums, dp, mask_next);

                dp.insert((index_target, mask_next), temp);
                permutation_amount = (permutation_amount + temp) % MOD;
            }
        }

        permutation_amount
    }
}

#[test]
fn test() {
    assert_eq!(Solution::special_perm(vec![2, 3, 6]), 2);
    assert_eq!(Solution::special_perm(vec![1, 4, 3]), 2);
}
