#![allow(dead_code)]

// 3434. Maximum Frequency After Subarray Operation
// https://leetcode.com/problems/maximum-frequency-after-subarray-operation/
// https://leetcode.cn/problems/maximum-frequency-after-subarray-operation/
//
// Medium
//
// You are given an array nums of length n. You are also given an integer k.
//
// You perform the following operation on nums once:
//
// - Select a subarray nums[i..j] where 0 <= i <= j <= n - 1.
// - Select an integer x and add x to all the elements in nums[i..j].
//
// Find the maximum frequency of the value k after the operation.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5,6], k = 1
//
// Output: 2
//
// Explanation:
//
// After adding -5 to nums[2..5], 1 has a frequency of 2 in [1, 2, -2, -1, 0, 1].
//
// Example 2:
//
// Input: nums = [10,2,3,4,5,5,4,3,2,2], k = 10
//
// Output: 4
//
// Explanation:
//
// After adding 8 to nums[1..9], 10 has a frequency of 4 in [10, 10, 11, 12, 13, 13, 12, 11, 10, 10].
//
// Constraints:
//
//     1 <= n == nums.length <= 10^5
//     1 <= nums[i] <= 50
//     1 <= k <= 50
//

struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        fn f(x: i32, nums: &[i32], k: i32) -> i32 {
            let mut a = 0;
            let mut b = 0;
            let mut best = 0;
            for &y in nums.iter() {
                if y == x {
                    b += 1
                }
                if y == k {
                    b -= 1
                }
                best = best.max(b - a);
                a = a.min(b)
            }
            best
        }

        let s = nums.iter().filter(|&&x| x == k).count() as i32;
        s + (1..=51).map(|x| f(x, &nums, k)).max().unwrap()
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let k = 1;
    let output = 2;
    assert_eq!(Solution::max_frequency(nums, k), output);

    let nums = vec![10, 2, 3, 4, 5, 5, 4, 3, 2, 2];
    let k = 10;
    let output = 4;
    assert_eq!(Solution::max_frequency(nums, k), output);
}
