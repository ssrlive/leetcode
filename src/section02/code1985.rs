#![allow(dead_code)]

/*

// 1985. Find the Kth Largest Integer in the Array
// https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/
// https://leetcode.cn/problems/find-the-kth-largest-integer-in-the-array/
//
// Medium
//
// You are given an array of strings nums and an integer k. Each string in nums represents an integer without leading zeros.

Return the string that represents the kth largest integer in nums.

Note: Duplicate numbers should be counted distinctly. For example, if nums is ["1","2","2"], "2" is the first largest integer, "2" is the second-largest integer, and "1" is the third-largest integer.

Example 1:

Input: nums = ["3","6","7","10"], k = 4
Output: "3"
Explanation:
The numbers in nums sorted in non-decreasing order are ["3","6","7","10"].
The 4th largest integer in nums is "3".

Example 2:

Input: nums = ["2","21","12","1"], k = 3
Output: "2"
Explanation:
The numbers in nums sorted in non-decreasing order are ["1","2","12","21"].
The 3rd largest integer in nums is "2".

Example 3:

Input: nums = ["0","0"], k = 2
Output: "0"
Explanation:
The numbers in nums sorted in non-decreasing order are ["0","0"].
The 2nd largest integer in nums is "0".

Constraints:

    1 <= k <= nums.length <= 10^4
    1 <= nums[i].length <= 100
    nums[i] consists of only digits.
    nums[i] will not have any leading zeros.
*/

struct Solution;

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        use std::cmp::Ordering;
        let mut nums = nums;
        nums.sort_by(|a, b| {
            let v = a.len().cmp(&b.len());
            if v == Ordering::Equal {
                a.cmp(b)
            } else {
                v
            }
        });
        nums.reverse();
        nums[k as usize - 1].to_string()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["3", "6", "7", "10"], 4, "3"),
        (vec!["2", "21", "12", "1"], 3, "2"),
        (vec!["0", "0"], 2, "0"),
    ];
    for (nums, k, expect) in cases {
        let nums = nums.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::kth_largest_number(nums, k), expect);
    }
}
