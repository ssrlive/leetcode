#![allow(dead_code)]

// 698. Partition to K Equal Sum Subsets
// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
// https://leetcode.cn/problems/partition-to-k-equal-sum-subsets/
//
// Given an integer array nums and an integer k, return true if it is possible to divide this array
// into k non-empty subsets whose sums are all equal.
//
// Example 1:
//
// Input: nums = [4,3,2,3,5,2,1], k = 4
// Output: true
// Explanation: It is possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
//
// Example 2:
//
// Input: nums = [1,2,3,4], k = 3
// Output: false
//
// Constraints:
//
// - 1 <= k <= nums.length <= 16
// - 1 <= nums[i] <= 10^4
// - The frequency of each element is in the range [1, 4].
//

struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let len_n = nums.len();
        let range = 1 << len_n;
        let sum_all: i32 = nums.iter().sum();
        if sum_all % k != 0 {
            return false;
        }
        let mut sums = vec![0; range];
        let mut dp = {
            let mut tmp = vec![false; range];
            tmp[0] = true;
            tmp
        };
        let target = sum_all / k;
        let sorted = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        if *sorted.last().unwrap_or(&(target + 1)) > target {
            return false;
        };
        for cur in 0..range {
            if dp[cur] {
                for (idx, &num) in sorted.iter().enumerate() {
                    let next = cur | (1 << idx);
                    if cur == next {
                        continue;
                    }
                    if num <= target - sums[cur] % target {
                        dp[next] = true;
                        sums[next] = num + sums[cur];
                    } else {
                        break;
                    }
                }
            }
        }
        dp[range - 1]
    }
}

#[test]
fn test() {
    let nums = vec![6, 5, 9, 6, 3, 5, 1, 10, 4, 1, 4, 3, 9, 9, 3, 3];
    assert!(!Solution::can_partition_k_subsets(nums, 9));

    assert!(Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4));
    assert!(!Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3));

    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    assert!(Solution::can_partition_k_subsets(nums, 4));
}
