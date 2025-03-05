#![allow(dead_code)]

// 3468. Find the Number of Copy Arrays
// https://leetcode.com/problems/find-the-number-of-copy-arrays/
// https://leetcode.cn/problems/find-the-number-of-copy-arrays/
//
// Medium
//
// You are given an array original of length n and a 2D array bounds of length n x 2, where bounds[i] = [ui, vi].
//
// You need to find the number of possible arrays copy of length n such that:
//
// (copy[i] - copy[i - 1]) == (original[i] - original[i - 1]) for 1 <= i <= n - 1.
// ui <= copy[i] <= vi for 0 <= i <= n - 1.
// Return the number of such arrays.
//
// Example 1:
//
// Input: original = [1,2,3,4], bounds = [[1,2],[2,3],[3,4],[4,5]]
//
// Output: 2
//
// Explanation:
//
// The possible arrays are:
//
// [1, 2, 3, 4]
// [2, 3, 4, 5]
//
// Example 2:
//
// Input: original = [1,2,3,4], bounds = [[1,10],[2,9],[3,8],[4,7]]
//
// Output: 4
//
// Explanation:
//
// The possible arrays are:
//
// [1, 2, 3, 4]
// [2, 3, 4, 5]
// [3, 4, 5, 6]
// [4, 5, 6, 7]
//
// Example 3:
//
// Input: original = [1,2,1,2], bounds = [[1,1],[2,3],[3,3],[2,3]]
//
// Output: 0
//
// Explanation:
//
// No array is possible.
//
// Constraints:
//
// 2 <= n == original.length <= 10^5
// 1 <= original[i] <= 10^9
// bounds.length == n
// bounds[i].length == 2
// 1 <= bounds[i][0] <= bounds[i][1] <= 10^9
//

struct Solution;

impl Solution {
    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let (mut low, mut high) = (bounds[0][0], bounds[0][1]);
        let mut ans = high - low + 1;
        for i in 1..original.len() {
            let diff = original[i] - original[i - 1];
            low = std::cmp::max(low + diff, bounds[i][0]);
            high = std::cmp::min(high + diff, bounds[i][1]);
            ans = std::cmp::min(ans, high - low + 1);
        }
        if ans < 0 {
            0
        } else {
            ans
        }
    }
}

#[test]
fn test() {
    let original = vec![1, 2, 3, 4];
    let bounds = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
    let output = 2;
    assert_eq!(Solution::count_arrays(original, bounds), output);

    let original = vec![1, 2, 3, 4];
    let bounds = vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7]];
    let output = 4;
    assert_eq!(Solution::count_arrays(original, bounds), output);

    let original = vec![1, 2, 1, 2];
    let bounds = vec![vec![1, 1], vec![2, 3], vec![3, 3], vec![2, 3]];
    let output = 0;
    assert_eq!(Solution::count_arrays(original, bounds), output);
}
