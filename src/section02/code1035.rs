#![allow(dead_code)]

// 1035. Uncrossed Lines
// https://leetcode.com/problems/uncrossed-lines/
// https://leetcode.cn/problems/uncrossed-lines/
//
// You are given two integer arrays nums1 and nums2. We write the integers of nums1 and nums2 (in the order they are given) on two separate horizontal lines.
//
// We may draw connecting lines: a straight line connecting two numbers nums1[i] and nums2[j] such that:
//
// nums1[i] == nums2[j], and
// the line we draw does not intersect any other connecting (non-horizontal) line.
// Note that a connecting line cannot intersect even at the endpoints (i.e., each number can only belong to one connecting line).
//
// Return the maximum number of connecting lines we can draw in this way.
//
// Example 1:
//
// Input: nums1 = [1,4,2], nums2 = [1,2,4]
// Output: 2
// Explanation: We can draw 2 uncrossed lines as in the diagram.
// We cannot draw 3 uncrossed lines, because the line from nums1[1] = 4 to nums2[2] = 4 will intersect the line from nums1[2]=2 to nums2[1]=2.
//
// Example 2:
//
// Input: nums1 = [2,5,1,2,5], nums2 = [10,5,2,1,5,2]
// Output: 3
//
// Example 3:
//
// Input: nums1 = [1,3,7,1,7,5], nums2 = [1,9,2,5,1]
// Output: 2
//
// Constraints:
//
// - 1 <= nums1.length, nums2.length <= 500
// - 1 <= nums1[i], nums2[j] <= 2000
//

struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[n][m]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 4, 2], vec![1, 2, 4], 2),
        (vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2], 3),
        (vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1], 2),
    ];
    for (nums1, nums2, expected) in cases {
        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), expected);
    }
}
