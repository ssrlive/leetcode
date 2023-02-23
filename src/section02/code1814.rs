#![allow(dead_code)]

/*

// 1814. Count Nice Pairs in an Array
// https://leetcode.com/problems/count-nice-pairs-in-an-array/
// https://leetcode.cn/problems/count-nice-pairs-in-an-array/
//
// Medium
//
// You are given an array nums that consists of non-negative integers. Let us define rev(x) as the reverse of the non-negative integer x. For example, rev(123) = 321, and rev(120) = 21. A pair of indices (i, j) is nice if it satisfies all of the following conditions:

    0 <= i < j < nums.length
    nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])

Return the number of nice pairs of indices. Since that number can be too large, return it modulo 109 + 7.

Example 1:

Input: nums = [42,11,1,97]
Output: 2
Explanation: The two pairs are:
 - (0,3) : 42 + rev(97) = 42 + 79 = 121, 97 + rev(42) = 97 + 24 = 121.
 - (1,2) : 11 + rev(1) = 11 + 1 = 12, 1 + rev(11) = 1 + 11 = 12.

Example 2:

Input: nums = [13,10,35,24,76]
Output: 4

Constraints:

    1 <= nums.length <= 10^5
    0 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn rev(mut num: i32) -> i32 {
            let mut res = 0;
            while num > 0 {
                res *= 10;
                res += num % 10;
                num /= 10;
            }
            res
        }

        let mut dist = HashMap::new();
        for num in nums {
            let offset = num - rev(num);
            *dist.entry(offset).or_insert(0) += 1;
        }
        let mut res: i64 = dist
            .into_iter()
            .filter(|&(_, ct)| ct > 1)
            .map(|(_, ct)| ct as i64)
            .map(|ct| ct * (ct - 1) / 2)
            .sum();
        res %= 1_000_000_007;
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
    assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
}
