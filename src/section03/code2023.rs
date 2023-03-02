#![allow(dead_code)]

/*

// 2023. Number of Pairs of Strings With Concatenation Equal to Target
// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
// https://leetcode.cn/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
//
// Medium
//
// Given an array of digit strings nums and a digit string target, return the number of pairs of indices (i, j) (where i != j) such that the concatenation of nums[i] + nums[j] equals target.

Example 1:

Input: nums = ["777","7","77","77"], target = "7777"
Output: 4
Explanation: Valid pairs are:
- (0, 1): "777" + "7"
- (1, 0): "7" + "777"
- (2, 3): "77" + "77"
- (3, 2): "77" + "77"

Example 2:

Input: nums = ["123","4","12","34"], target = "1234"
Output: 2
Explanation: Valid pairs are:
- (0, 1): "123" + "4"
- (2, 3): "12" + "34"

Example 3:

Input: nums = ["1","1","1"], target = "11"
Output: 6
Explanation: Valid pairs are:
- (0, 1): "1" + "1"
- (1, 0): "1" + "1"
- (0, 2): "1" + "1"
- (2, 0): "1" + "1"
- (1, 2): "1" + "1"
- (2, 1): "1" + "1"

Constraints:

    2 <= nums.length <= 100
    1 <= nums[i].length <= 100
    2 <= target.length <= 100
    nums[i] and target consist of digits.
    nums[i] and target do not have leading zeros.
*/

struct Solution;

impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j && nums[i].to_owned() + &nums[j] == target {
                    count += 1;
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["777", "7", "77", "77"], "7777", 4),
        (vec!["123", "4", "12", "34"], "1234", 2),
        (vec!["1", "1", "1"], "11", 6),
    ];
    for (nums, target, expected) in cases {
        let nums = nums.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::num_of_pairs(nums, target.to_string()), expected);
    }
}
