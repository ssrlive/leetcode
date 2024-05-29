#![allow(dead_code)]

// 3165. Maximum Sum of Subsequence With Non-adjacent Elements
// https://leetcode.com/problems/maximum-sum-of-subsequence-with-non-adjacent-elements/
// https://leetcode.cn/problems/maximum-sum-of-subsequence-with-non-adjacent-elements/
//
// Hard
//
// You are given an array nums consisting of integers. You are also given a 2D array queries, where queries[i] = [posi, xi].
//
// For query i, we first set nums[posi] equal to xi, then we calculate the answer to query i which is the maximum sum of a
// subsequence of nums where no two adjacent elements are selected.
//
// Return the sum of the answers to all queries.
//
// Since the final answer may be very large, return it modulo 109 + 7.
//
// A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: nums = [3,5,9], queries = [[1,-2],[0,-3]]
//
// Output: 21
//
// Explanation:
// After the 1st query, nums = [3,-2,9] and the maximum sum of a subsequence with non-adjacent elements is 3 + 9 = 12.
// After the 2nd query, nums = [-3,-2,9] and the maximum sum of a subsequence with non-adjacent elements is 9.
//
// Example 2:
//
// Input: nums = [0,-1], queries = [[0,-5]]
//
// Output: 0
//
// Explanation:
// After the 1st query, nums = [-5,-1] and the maximum sum of a subsequence with non-adjacent elements is 0 (choosing an empty subsequence).
//
// Constraints:
//
// 1 <= nums.length <= 5 * 10^4
// -10^5 <= nums[i] <= 10^5
// 1 <= queries.length <= 5 * 10^4
// queries[i] == [posi, xi]
// 0 <= posi <= nums.length - 1
// -10^5 <= xi <= 10^5
//

struct Solution;

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        let mut ans: i64 = 0;
        let part_size = 8.max((nums.len() as f64).sqrt() as usize);
        let mut parts: Vec<[i32; 4]> = Vec::new();

        for i in (0..nums.len()).step_by(part_size) {
            parts.push(Self::calculate_max_non_adjacent_sum(
                &nums[i..(i + part_size).min(nums.len())],
                0,
                part_size,
            ));
        }

        for query in queries {
            let part_index = query[0] as usize / part_size;
            nums[query[0] as usize] = query[1];
            parts[part_index] = Self::calculate_max_non_adjacent_sum(
                &nums[part_index * part_size..((part_index + 1) * part_size).min(nums.len())],
                0,
                part_size,
            );
            ans += Self::combine_parts(&parts) as i64;
            ans %= Self::MOD;
        }

        ans as i32
    }

    const MOD: i64 = 1_000_000_007;

    fn calculate_max_non_adjacent_sum(nums: &[i32], start: usize, end: usize) -> [i32; 4] {
        let mut first_last = 0;
        let mut first_second_last = 0;
        let mut second_last = 0;
        let mut second_second_last = 0;

        for (i, &nums_i) in nums.iter().enumerate().take(nums.len().min(end)).skip(start) {
            first_last = std::cmp::max(std::mem::replace(&mut first_second_last, first_last) + nums_i, first_last);
            if i > start {
                second_last = std::cmp::max(std::mem::replace(&mut second_second_last, second_last) + nums_i, second_last);
            }
        }

        [first_last, first_second_last, second_last, second_second_last]
    }

    fn combine_parts(parts: &[[i32; 4]]) -> i32 {
        let mut max_last = 0;
        let mut max_second_last = 0;

        for &[first_last, first_second_last, second_last, second_second_last] in parts {
            let temp = max_last;
            max_last = std::cmp::max(max_last + second_last, max_second_last + first_last);
            max_second_last = std::cmp::max(temp + second_second_last, max_second_last + first_second_last);
        }

        max_last
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_sum_subsequence(vec![3, 5, 9], vec![vec![1, -2], vec![0, -3]]), 21);
    assert_eq!(Solution::maximum_sum_subsequence(vec![0, -1], vec![vec![0, -5]]), 0);
}
