#![allow(dead_code)]

// 2518. Number of Great Partitions
// https://leetcode.com/problems/number-of-great-partitions/
// https://leetcode.cn/problems/number-of-great-partitions/
//
// You are given an array nums consisting of positive integers and an integer k.
//
// Partition the array into two ordered groups such that each element is in exactly one group.
// A partition is called great if the sum of elements of each group is greater than or equal to k.
//
// Return the number of distinct great partitions. Since the answer may be too large, return it modulo 109 + 7.
//
// Two partitions are considered distinct if some element nums[i] is in different groups in the two partitions.
//
// Example 1:
//
// Input: nums = [1,2,3,4], k = 4
// Output: 6
// Explanation: The great partitions are: ([1,2,3], [4]), ([1,3], [2,4]), ([1,4], [2,3]), ([2,3], [1,4]), ([2,4], [1,3]) and ([4], [1,2,3]).
//
// Example 2:
//
// Input: nums = [3,3,3], k = 4
// Output: 0
// Explanation: There are no great partitions for this array.
//
// Example 3:
//
// Input: nums = [6,6], k = 2
// Output: 2
// Explanation: We can either put nums[0] in the first partition or in the second partition.
// The great partitions will be ([6], [6]) and ([6], [6]).
//
// Constraints:
//
// - 1 <= nums.length, k <= 1000
// - 1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        fn dfs(dp: &mut Vec<Vec<i64>>, nums: &Vec<i64>, k: i64, i: usize, sum: i64) -> i64 {
            if sum >= k || i == nums.len() {
                return i64::from(sum > 0 && sum < k);
            }
            if dp[i][sum as usize] == 0 {
                dp[i][sum as usize] =
                    (1 + dfs(dp, nums, k, i + 1, sum) + dfs(dp, nums, k, i + 1, sum + nums[i])) % 1_000_000_007;
            }
            dp[i][sum as usize] - 1
        }

        fn _count_partitions(nums: Vec<i64>, k: i64) -> i64 {
            let mut cnt = vec![0; 1001];
            let mut dp = vec![vec![0; 1000]; 1000];
            if cnt[2] == 0 {
                for i in 2..1001 {
                    cnt[i] = 2 * (cnt[i - 1] + 1) % 1_000_000_007;
                }
            }
            if nums.iter().sum::<i64>() < k * 2 {
                return 0;
            }
            (1_000_000_007 + cnt[nums.len()] - 2 * dfs(&mut dp, &nums, k, 0, 0)) % 1_000_000_007
        }

        _count_partitions(nums.iter().map(|&x| i64::from(x)).collect(), i64::from(k)) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_partitions(vec![1, 2, 3, 4], 4), 6);
    assert_eq!(Solution::count_partitions(vec![3, 3, 3], 4), 0);
    assert_eq!(Solution::count_partitions(vec![6, 6], 2), 2);
}
