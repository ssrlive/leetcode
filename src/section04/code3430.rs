#![allow(dead_code)]

// 3430. Maximum and Minimum Sums of at Most Size K Subarrays
// https://leetcode.com/problems/maximum-and-minimum-sums-of-at-most-size-k-subarrays/
// https://leetcode.cn/problems/maximum-and-minimum-sums-of-at-most-size-k-subarrays/
//
// Hard
//
// You are given an integer array nums and a positive integer k. Return the sum of the maximum and
// minimum elements of all subarrays with at most k elements.
//
// Example 1:
//
// Input: nums = [1,2,3], k = 2
//
// Output: 20
//
// Explanation:
//
// The subarrays of nums with at most 2 elements are:
// Subarray	Minimum	Maximum	Sum
// [1]	1	1	2
// [2]	2	2	4
// [3]	3	3	6
// [1, 2]	1	2	3
// [2, 3]	2	3	5
// Final Total	 	 	20
//
// The output would be 20.
//
// Example 2:
//
// Input: nums = [1,-3,1], k = 2
//
// Output: -6
//
// Explanation:
//
// The subarrays of nums with at most 2 elements are:
// Subarray	Minimum	Maximum	Sum
// [1]	1	1	2
// [-3]	-3	-3	-6
// [1]	1	1	2
// [1, -3]	-3	1	-2
// [-3, 1]	-3	1	-2
// Final Total	 	 	-6
//
// The output would be -6.
//
// Constraints:
//
//     1 <= nums.length <= 80000
//     1 <= k <= nums.length
//     -10^6 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        let k = k as i64;
        let mut res = 0;
        let mut total = 0;
        let mut dq: std::collections::VecDeque<(i64, i64)> = std::collections::VecDeque::new();
        let n = nums.len() as i64;
        let mut l = 0_i64;

        for r in 0..n {
            if r - l + 1 > k {
                total -= nums[dq.front().unwrap().0 as usize];
                dq.front_mut().unwrap().1 -= 1;
                l += 1;
            }
            while !dq.is_empty() && dq.front().unwrap().0 < l {
                total -= dq.front().unwrap().1 * nums[dq.front().unwrap().0 as usize];
                dq.pop_front();
            }
            while !dq.is_empty() && nums[dq.back().unwrap().0 as usize] < nums[r as usize] {
                total -= dq.back().unwrap().1 * nums[dq.back().unwrap().0 as usize];
                dq.pop_back();
            }

            let cnt = if dq.is_empty() {
                std::cmp::min(r - l + 1, k)
            } else {
                r - dq.back().unwrap().0
            };
            total += cnt * nums[r as usize];
            res += total;

            dq.push_back((r, cnt));
        }

        l = 0;
        dq.clear();
        total = 0;

        for r in 0..n {
            if r - l + 1 > k {
                total -= nums[dq.front().unwrap().0 as usize];
                dq.front_mut().unwrap().1 -= 1;
                l += 1;
            }
            while !dq.is_empty() && dq.front().unwrap().0 < l {
                total -= dq.front().unwrap().1 * nums[dq.front().unwrap().0 as usize];
                dq.pop_front();
            }
            while !dq.is_empty() && nums[dq.back().unwrap().0 as usize] > nums[r as usize] {
                total -= dq.back().unwrap().1 * nums[dq.back().unwrap().0 as usize];
                dq.pop_back();
            }

            let cnt = if dq.is_empty() {
                std::cmp::min(r - l + 1, k)
            } else {
                r - dq.back().unwrap().0
            };
            total += cnt * nums[r as usize];
            res += total;

            dq.push_back((r, cnt));
        }

        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let k = 2;
    let res = 20;
    assert_eq!(Solution::min_max_subarray_sum(nums, k), res);

    let nums = vec![1, -3, 1];
    let k = 2;
    let res = -6;
    assert_eq!(Solution::min_max_subarray_sum(nums, k), res);
}
