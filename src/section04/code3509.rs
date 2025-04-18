#![allow(dead_code)]

// 3509. Maximum Product of Subsequences With an Alternating Sum Equal to K
// https://leetcode.com/problems/maximum-product-of-subsequences-with-an-alternating-sum-equal-to-k/
// https://leetcode.cn/problems/maximum-product-of-subsequences-with-an-alternating-sum-equal-to-k/
//
// Hard
//
// You are given an integer array nums and two integers, k and limit. Your task is to find a non-empty
//
// of nums that:
//
//     Has an alternating sum equal to k.
//     Maximizes the product of all its numbers without the product exceeding limit.
//
// Return the product of the numbers in such a subsequence. If no subsequence satisfies the requirements, return -1.
//
// The alternating sum of a 0-indexed array is defined as the sum of the elements at even indices minus the sum of the elements at odd indices.
//
// Example 1:
//
// Input: nums = [1,2,3], k = 2, limit = 10
//
// Output: 6
//
// Explanation:
//
// The subsequences with an alternating sum of 2 are:
//
//     [1, 2, 3]
//         Alternating Sum: 1 - 2 + 3 = 2
//         Product: 1 * 2 * 3 = 6
//     [2]
//         Alternating Sum: 2
//         Product: 2
//
// The maximum product within the limit is 6.
//
// Example 2:
//
// Input: nums = [0,2,3], k = -5, limit = 12
//
// Output: -1
//
// Explanation:
//
// A subsequence with an alternating sum of exactly -5 does not exist.
//
// Example 3:
//
// Input: nums = [2,2,3,3], k = 0, limit = 9
//
// Output: 9
//
// Explanation:
//
// The subsequences with an alternating sum of 0 are:
//
//     [2, 2]
//         Alternating Sum: 2 - 2 = 0
//         Product: 2 * 2 = 4
//     [3, 3]
//         Alternating Sum: 3 - 3 = 0
//         Product: 3 * 3 = 9
//     [2, 2, 3, 3]
//         Alternating Sum: 2 - 2 + 3 - 3 = 0
//         Product: 2 * 2 * 3 * 3 = 36
//
// The subsequence [2, 2, 3, 3] has the greatest product with an alternating sum equal to k, but 36 > 9. The next greatest product is 9, which is within the limit.
//
// Constraints:
//
//     1 <= nums.length <= 150
//     0 <= nums[i] <= 12
//     -10^5 <= k <= 10^5
//     1 <= limit <= 5000
//

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        use std::collections::HashMap;
        #[allow(clippy::too_many_arguments, clippy::type_complexity)]
        fn recursion(
            pos: usize,
            curr_sum: i32,
            product: i32,
            is_odd: i32,
            k: i32,
            n: usize,
            nums: &[i32],
            limit: i32,
            dp: &mut HashMap<i32, HashMap<i32, HashMap<i32, HashMap<i32, i32>>>>,
        ) -> i32 {
            if pos == n {
                return if curr_sum == k && is_odd != 0 && product <= limit {
                    product
                } else {
                    i32::MIN
                };
            }

            let pos_i32 = pos as i32;
            if let Some(pos_map) = dp.get(&pos_i32) {
                if let Some(curr_sum_map) = pos_map.get(&curr_sum) {
                    if let Some(product_map) = curr_sum_map.get(&product) {
                        if let Some(is_odd_map) = product_map.get(&is_odd) {
                            return *is_odd_map;
                        }
                    }
                }
            }

            let mut ans = recursion(pos + 1, curr_sum, product, is_odd, k, n, nums, limit, dp);
            if is_odd == 0 {
                ans = ans.max(recursion(pos + 1, curr_sum + nums[pos], nums[pos], 2, k, n, nums, limit, dp));
            }
            if is_odd == 1 {
                ans = ans.max(recursion(
                    pos + 1,
                    curr_sum + nums[pos],
                    (product * nums[pos]).min(limit + 1),
                    2,
                    k,
                    n,
                    nums,
                    limit,
                    dp,
                ));
            }
            if is_odd == 2 {
                ans = ans.max(recursion(
                    pos + 1,
                    curr_sum - nums[pos],
                    (product * nums[pos]).min(limit + 1),
                    1,
                    k,
                    n,
                    nums,
                    limit,
                    dp,
                ));
            }

            dp.entry(pos_i32)
                .or_default()
                .entry(curr_sum)
                .or_default()
                .entry(product)
                .or_default()
                .entry(is_odd)
                .or_insert(ans);

            ans
        }

        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        if k > sum || k < -sum {
            return -1;
        }

        #[allow(clippy::type_complexity)]
        let mut dp: HashMap<i32, HashMap<i32, HashMap<i32, HashMap<i32, i32>>>> = HashMap::new();
        let ans = recursion(0, 0, 0, 0, k, n, &nums, limit, &mut dp);
        if ans == i32::MIN { -1 } else { ans }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_product(vec![1, 2, 3], 2, 10), 6);
    assert_eq!(Solution::max_product(vec![0, 2, 3], -5, 12), -1);
    assert_eq!(Solution::max_product(vec![2, 2, 3, 3], 0, 9), 9);
}
