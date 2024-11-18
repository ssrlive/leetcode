#![allow(dead_code)]

// 3355. Zero Array Transformation I
// https://leetcode.com/problems/zero-array-transformation-i/
// https://leetcode.cn/problems/zero-array-transformation-i/
//
// Medium
//
// You are given an integer array nums of length n and a 2D array queries, where queries[i] = [li, ri].
//
// For each queries[i]:
//
// Select a subset of indices within the range [li, ri] in nums.
// Decrement the values at the selected indices by 1.
// A Zero Array is an array where all elements are equal to 0.
//
// Return true if it is possible to transform nums into a Zero Array after processing all the queries sequentially, otherwise return false.
//
// A subset of an array is a selection of elements (possibly none) of the array.
//
// Example 1:
//
// Input: nums = [1,0,1], queries = [[0,2]]
//
// Output: true
//
// Explanation:
//
// For i = 0:
// Select the subset of indices as [0, 2] and decrement the values at these indices by 1.
// The array will become [0, 0, 0], which is a Zero Array.
//
// Example 2:
//
// Input: nums = [4,3,2,1], queries = [[1,3],[0,2]]
//
// Output: false
//
// Explanation:
//
// For i = 0:
// Select the subset of indices as [1, 2, 3] and decrement the values at these indices by 1.
// The array will become [4, 2, 1, 0].
// For i = 1:
// Select the subset of indices as [0, 1, 2] and decrement the values at these indices by 1.
// The array will become [3, 1, 0, 0], which is not a Zero Array.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 0 <= nums[i] <= 10^5
// 1 <= queries.length <= 10^5
// queries[i].length == 2
// 0 <= li <= ri < nums.length
//

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let (mut a, mut b): (Vec<_>, Vec<_>) = queries.iter().map(|x| (x[0], x[1])).unzip();
        a.sort_unstable();
        b.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        let mut w = 0;
        for (k, x) in nums.into_iter().enumerate() {
            while i < a.len() && a[i] <= k as i32 {
                w += 1;
                i += 1;
            }
            while j < b.len() && b[j] < k as i32 {
                w -= 1;
                j += 1;
            }
            if x > w {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, 1];
    let queries = vec![vec![0, 2]];
    let res = true;
    assert_eq!(Solution::is_zero_array(nums, queries), res);

    let nums = vec![4, 3, 2, 1];
    let queries = vec![vec![1, 3], vec![0, 2]];
    let res = false;
    assert_eq!(Solution::is_zero_array(nums, queries), res);
}
