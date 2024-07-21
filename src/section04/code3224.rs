#![allow(dead_code)]

// 3224. Minimum Array Changes to Make Differences Equal
// https://leetcode.com/problems/minimum-array-changes-to-make-differences-equal/
// https://leetcode.cn/problems/minimum-array-changes-to-make-differences-equal/
//
// Medium
//
// You are given an integer array nums of size n where n is even, and an integer k.
//
// You can perform some changes on the array, where in one change you can replace any element
// in the array with any integer in the range from 0 to k.
//
// You need to perform some changes (possibly none) such that the final array satisfies the following condition:
//
//     There exists an integer X such that abs(a[i] - a[n - i - 1]) = X for all (0 <= i < n).
//
// Return the minimum number of changes required to satisfy the above condition.
//
// Example 1:
//
// Input: nums = [1,0,1,2,4,3], k = 4
//
// Output: 2
//
// Explanation:
// We can perform the following changes:
//
//     Replace nums[1] by 2. The resulting array is nums = [1,2,1,2,4,3].
//     Replace nums[3] by 3. The resulting array is nums = [1,2,1,3,4,3].
//
// The integer X will be 2.
//
// Example 2:
//
// Input: nums = [0,1,2,3,3,6,5,4], k = 6
//
// Output: 2
//
// Explanation:
// We can perform the following operations:
//
//     Replace nums[3] by 0. The resulting array is nums = [0,1,2,0,3,6,5,4].
//     Replace nums[4] by 4. The resulting array is nums = [0,1,2,0,4,6,5,4].
//
// The integer X will be 4.
//
// Constraints:
//
//     2 <= n == nums.length <= 10^5
//     n is even.
//     0 <= nums[i] <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = vec![0; (k + 1) as usize];
        let mut v = Vec::new();
        let mut n = nums.len();
        for i in 0..n / 2 {
            let dif = (nums[i] - nums[n - i - 1]).unsigned_abs() as usize;
            freq[dif] += 1;

            let a = nums[i];
            let b = nums[n - i - 1];
            let threshold = std::cmp::max(a.max(b), k - a.min(b));
            v.push(threshold);
        }

        v.sort_unstable();

        let mut ans = n / 2;
        n /= 2;
        for (i, &freq_i) in freq.iter().enumerate() {
            let rest = n - freq_i;
            let i = i as i32;
            // let greater = v.binary_search(&i).unwrap_or_else(|e| e);
            let greater = v.partition_point(|&x| x < i);
            ans = std::cmp::min(ans, rest + greater);
        }
        ans as _
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, 1, 2, 4, 3];
    let k = 4;
    let output = 2;
    assert_eq!(Solution::min_changes(nums, k), output);

    let nums = vec![0, 1, 2, 3, 3, 6, 5, 4];
    let k = 6;
    let output = 2;
    assert_eq!(Solution::min_changes(nums, k), output);

    // [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] 0 => 0
    let nums = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let k = 0;
    let output = 0;
    assert_eq!(Solution::min_changes(nums, k), output);
}
