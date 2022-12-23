#![allow(dead_code)]

// 42. Trapping Rain Water
// https://leetcode.com/problems/trapping-rain-water/
// https://leetcode.cn/problems/trapping-rain-water/
//
// Given n non-negative integers representing an elevation map where the width of each bar is 1,
// compute how much water it can trap after raining.
//
// Example 1:
//
// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6
// Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1].
// In this case, 6 units of rain water (blue section) are being trapped.
//
// Example 2:
//
// Input: height = [4,2,0,3,2,5]
// Output: 9
//
// Constraints:
//
// - n == height.length
// - 1 <= n <= 2 * 10^4
// - 0 <= height[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut v = vec![i32::MAX; height.len()];
        {
            let mut max = 0;
            for (i, &h) in height.iter().enumerate() {
                max = max.max(h);
                v[i] = v[i].min(max);
            }
        }
        {
            let mut max = 0;
            for (i, &h) in height.iter().enumerate().rev() {
                max = max.max(h);
                v[i] = v[i].min(max);
            }
        }
        height.iter().zip(&v).map(|(h, m)| m - h).sum()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}
