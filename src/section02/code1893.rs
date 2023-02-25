#![allow(dead_code)]

/*

// 1893. Check if All the Integers in a Range Are Covered
// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/
// https://leetcode.cn/problems/check-if-all-the-integers-in-a-range-are-covered/
//
// Easy
//
// You are given a 2D integer array ranges and two integers left and right. Each ranges[i] = [starti, endi] represents an inclusive interval between starti and endi.

Return true if each integer in the inclusive range [left, right] is covered by at least one interval in ranges. Return false otherwise.

An integer x is covered by an interval ranges[i] = [starti, endi] if starti <= x <= endi.

Example 1:

Input: ranges = [[1,2],[3,4],[5,6]], left = 2, right = 5
Output: true
Explanation: Every integer between 2 and 5 is covered:
- 2 is covered by the first range.
- 3 and 4 are covered by the second range.
- 5 is covered by the third range.

Example 2:

Input: ranges = [[1,10],[10,20]], left = 21, right = 21
Output: false
Explanation: 21 is not covered by any range.

Constraints:

    1 <= ranges.length <= 50
    1 <= starti <= endi <= 50
    1 <= left <= right <= 50
*/

struct Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        use std::collections::HashSet;
        let all_ranges = ranges.into_iter().flat_map(|r| r[0]..=r[1]).collect::<HashSet<_>>();
        (left..=right).all(|x| all_ranges.contains(&x))
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5, true),
        (vec![vec![1, 10], vec![10, 20]], 21, 21, false),
    ];
    for (ranges, left, right, expected) in cases {
        assert_eq!(Solution::is_covered(ranges, left, right), expected);
    }
}
