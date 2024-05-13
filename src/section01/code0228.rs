#![allow(dead_code)]

// 228. Summary Ranges
// https://leetcode.com/problems/summary-ranges/
// https://leetcode.cn/problems/summary-ranges/
//
// You are given a sorted unique integer array nums.
//
// A range [a,b] is the set of all integers from a to b (inclusive).
//
// Return the smallest sorted list of ranges that cover all the numbers in the array exactly.
// That is, each element of nums is covered by exactly one of the ranges,
// and there is no integer x such that x is in one of the ranges but not in nums.
//
// Each range [a,b] in the list should be output as:
//
// - "a->b" if a != b
// - "a" if a == b
//
// Example 1:
//
// Input: nums = [0,1,2,4,5,7]
// Output: ["0->2","4->5","7"]
// Explanation: The ranges are:
// [0,2] --> "0->2"
// [4,5] --> "4->5"
// [7,7] --> "7"
//
// Example 2:
//
// Input: nums = [0,2,3,4,6,8,9]
// Output: ["0","2->4","6","8->9"]
// Explanation: The ranges are:
// [0,0] --> "0"
// [2,4] --> "2->4"
// [6,6] --> "6"
// [8,9] --> "8->9"
//
// Constraints:
//
// - 0 <= nums.length <= 20
// - -231 <= nums[i] <= 231 - 1
// - All the values of nums are unique.
// - nums is sorted in ascending order.
//

struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return Vec::new();
        }

        let mut ranges = Vec::<(i32, i32)>::with_capacity(20);
        let [mut l, mut n, _] = [nums[0]; 3];
        for (&prev, &num) in nums.iter().zip(nums[1..].iter()) {
            let p = prev;
            n = num;
            if n != p + 1 {
                ranges.push((l, p));
                l = n;
            }
        }
        ranges.push((l, n));
        ranges
            .into_iter()
            .map(|(l, h)| if l == h { format!("{l}") } else { format!("{l}->{h}") })
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]), vec!["0->2", "4->5", "7"]);
    assert_eq!(Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]), vec!["0", "2->4", "6", "8->9"]);
}
