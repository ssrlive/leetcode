#![allow(dead_code)]

// 2420. Find All Good Indices
// https://leetcode.com/problems/find-all-good-indices/
// https://leetcode.cn/problems/find-all-good-indices/
//
// You are given a 0-indexed integer array nums of size n and a positive integer k.
//
// We call an index i in the range k <= i < n - k good if the following conditions are satisfied:
//
// The k elements that are just before the index i are in non-increasing order.
// The k elements that are just after the index i are in non-decreasing order.
// Return an array of all good indices sorted in increasing order.
//
// Example 1:
//
// Input: nums = [2,1,1,1,3,4,1], k = 2
// Output: [2,3]
// Explanation: There are two good indices in the array:
// - Index 2. The subarray [2,1] is in non-increasing order, and the subarray [1,3] is in non-decreasing order.
// - Index 3. The subarray [1,1] is in non-increasing order, and the subarray [3,4] is in non-decreasing order.
// Note that the index 4 is not good because [4,1] is not non-decreasing.
//
// Example 2:
//
// Input: nums = [2,1,1,2], k = 2
// Output: []
// Explanation: There are no good indices in this array.
//
// Constraints:
//
// - n == nums.length
// - 3 <= n <= 10^5
// - 1 <= nums[i] <= 10^6
// - 1 <= k <= n / 2
//

struct Solution;

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let (mut dec, mut inc) = (vec![0; n], vec![0; n]);

        for i in 1..n {
            dec[i] = 1;
            if i == 1 {
                continue;
            }
            if nums[i - 1] <= nums[i - 2] {
                dec[i] += dec[i - 1];
            }
        }

        for i in (0..n - 1).rev() {
            inc[i] = 1;
            if i == n - 2 {
                continue;
            }
            if nums[i + 1] <= nums[i + 2] {
                inc[i] += inc[i + 1];
            }
        }

        let mut ret = vec![];
        for i in 0..n {
            if inc[i] < k || dec[i] < k {
                continue;
            }
            ret.push(i as i32);
        }

        ret
    }
}

#[test]
fn test() {
    let nums = vec![2, 1, 1, 1, 3, 4, 1];
    let k = 2;
    let ret = Solution::good_indices(nums, k);
    assert_eq!(ret, vec![2, 3]);

    let nums = vec![2, 1, 1, 2];
    let k = 2;
    let ret = Solution::good_indices(nums, k);
    assert_eq!(ret, vec![]);
}
