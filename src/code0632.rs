#![allow(dead_code)]

// 632. Smallest Range Covering Elements from K Lists
// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/
// https://leetcode.cn/problems/smallest-range-covering-elements-from-k-lists/
//
// Hard
//
// You have k lists of sorted integers in non-decreasing order. Find the smallest range that includes at least one number from each of the k lists.
//
// We define the range [a, b] is smaller than range [c, d] if b - a < d - c or a < c if b - a == d - c.
//
// Example 1:
//
// Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
// Output: [20,24]
// Explanation:
// List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
// List 2: [0, 9, 12, 20], 20 is in range [20,24].
// List 3: [5, 18, 22, 30], 22 is in range [20,24].
//
// Example 2:
//
// Input: nums = [[1,2,3],[1,2,3],[1,2,3]]
// Output: [1,1]
//
// Constraints:
//
// nums.length == k
// 1 <= k <= 3500
// 1 <= nums[i].length <= 50
// -10^5 <= nums[i][j] <= 10^5
// nums[i] is sorted in non-decreasing order.
//

struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ranges = Vec::new();
        for (i, v) in nums.iter().enumerate() {
            for &n in v {
                ranges.push((n, i));
            }
        }
        ranges.sort();
        let mut map = std::collections::HashMap::new();
        let mut start = 0;
        let mut end = 0;
        let mut min = std::i32::MAX;
        let mut min_start = 0;
        let mut min_end = 0;
        while end < ranges.len() {
            map.entry(ranges[end].1).and_modify(|e| *e += 1).or_insert(1);
            while map.len() == nums.len() && map[&ranges[start].1] > 1 {
                map.entry(ranges[start].1).and_modify(|e| *e -= 1);
                start += 1;
            }
            if map.len() == nums.len() && min > ranges[end].0 - ranges[start].0 {
                min = ranges[end].0 - ranges[start].0;
                min_start = ranges[start].0;
                min_end = ranges[end].0;
            }
            end += 1;
        }
        vec![min_start, min_end]
    }
}

#[test]
fn test() {
    let nums = vec![vec![4, 10, 15, 24, 26], vec![0, 9, 12, 20], vec![5, 18, 22, 30]];
    let ans = vec![20, 24];
    assert_eq!(Solution::smallest_range(nums), ans);
    let nums = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
    let ans = vec![1, 1];
    assert_eq!(Solution::smallest_range(nums), ans);
}
