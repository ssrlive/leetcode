#![allow(dead_code)]

// 3551. Minimum Swaps to Sort by Digit Sum
// https://leetcode.com/problems/minimum-swaps-to-sort-by-digit-sum/
// https://leetcode.cn/problems/minimum-swaps-to-sort-by-digit-sum/
//
// Medium
//
// You are given an array nums of distinct positive integers. You need to sort the array
// in increasing order based on the sum of the digits of each number.
// If two numbers have the same digit sum, the smaller number appears first in the sorted order.
//
// Return the minimum number of swaps required to rearrange nums into this sorted order.
//
// A swap is defined as exchanging the values at two distinct positions in the array.
//
// Example 1:
//
// Input: nums = [37,100]
//
// Output: 1
//
// Explanation:
//
//     Compute the digit sum for each integer: [3 + 7 = 10, 1 + 0 + 0 = 1] → [10, 1]
//     Sort the integers based on digit sum: [100, 37]. Swap 37 with 100 to obtain the sorted order.
//     Thus, the minimum number of swaps required to rearrange nums is 1.
//
// Example 2:
//
// Input: nums = [22,14,33,7]
//
// Output: 0
//
// Explanation:
//
//     Compute the digit sum for each integer: [2 + 2 = 4, 1 + 4 = 5, 3 + 3 = 6, 7 = 7] → [4, 5, 6, 7]
//     Sort the integers based on digit sum: [22, 14, 33, 7]. The array is already sorted.
//     Thus, the minimum number of swaps required to rearrange nums is 0.
//
// Example 3:
//
// Input: nums = [18,43,34,16]
//
// Output: 2
//
// Explanation:
//
//     Compute the digit sum for each integer: [1 + 8 = 9, 4 + 3 = 7, 3 + 4 = 7, 1 + 6 = 7] → [9, 7, 7, 7]
//     Sort the integers based on digit sum: [16, 34, 43, 18]. Swap 18 with 16, and swap 43 with 34 to obtain the sorted order.
//     Thus, the minimum number of swaps required to rearrange nums is 2.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^9
//     nums consists of distinct positive integers.
//

struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        fn find_sum(mut n: i32) -> i32 {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            sum
        }

        let n = nums.len();
        let mut v = nums.clone();
        v.sort_by(|&a, &b| {
            let sum_a = find_sum(a);
            let sum_b = find_sum(b);
            let cmp = sum_a.cmp(&sum_b);
            if cmp == std::cmp::Ordering::Equal { a.cmp(&b) } else { cmp }
        });
        let mut mp = std::collections::HashMap::new();
        for (i, &nums_i) in nums.iter().enumerate() {
            mp.insert(nums_i, i);
        }
        let mut visited = vec![false; n];
        let mut cnt = 0;
        for i in 0..n {
            if visited[i] || v[i] == nums[i] {
                continue;
            }
            let mut cycle_size = 0;
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                j = *mp.get(&v[j]).unwrap();
                cycle_size += 1;
            }
            if cycle_size > 1 {
                cnt += cycle_size - 1;
            }
        }
        cnt
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_swaps(vec![37, 100]), 1);
    assert_eq!(Solution::min_swaps(vec![22, 14, 33, 7]), 0);
    assert_eq!(Solution::min_swaps(vec![18, 43, 34, 16]), 2);
}
