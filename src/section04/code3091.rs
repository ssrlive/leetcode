#![allow(dead_code)]

// 3091. Apply Operations to Make Sum of Array Greater Than or Equal to k
// https://leetcode.com/problems/apply-operations-to-make-sum-of-array-greater-than-or-equal-to-k/
// https://leetcode.cn/problems/apply-operations-to-make-sum-of-array-greater-than-or-equal-to-k/
//
// Medium
//
// You are given a positive integer k. Initially, you have an array nums = [1].
//
// You can perform any of the following operations on the array any number of times (possibly zero):
//
//     Choose any element in the array and increase its value by 1.
//     Duplicate any element in the array and add it to the end of the array.
//
// Return the minimum number of operations required to make the sum of elements of the final array greater than or equal to k.
//
// Example 1:
//
// Input: k = 11
//
// Output: 5
//
// Explanation:
//
// We can do the following operations on the array nums = [1]:
//
//     Increase the element by 1 three times. The resulting array is nums = [4].
//     Duplicate the element two times. The resulting array is nums = [4,4,4].
//
// The sum of the final array is 4 + 4 + 4 = 12 which is greater than or equal to k = 11.
// The total number of operations performed is 3 + 2 = 5.
//
// Example 2:
//
// Input: k = 1
//
// Output: 0
//
// Explanation:
//
// The sum of the original array is already greater than or equal to 1, so no operations are needed.
//
// Constraints:
//
//     1 <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn min_operations(k: i32) -> i32 {
        let mut best = 1_000_000_007;
        for t in 1..=k {
            let need = (k as f32 / t as f32).ceil() as i32;
            let cand = (t - 1) + (need - 1);
            best = best.min(cand);
        }
        best
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations(11), 5);
    assert_eq!(Solution::min_operations(1), 0);
}
