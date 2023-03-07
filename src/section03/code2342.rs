#![allow(dead_code)]

/*

// 2342. Max Sum of a Pair With Equal Sum of Digits
// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
// https://leetcode.cn/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
//
// Medium
//
// You are given a 0-indexed array nums consisting of positive integers. You can choose two indices i and j, such that i != j, and the sum of digits of the number nums[i] is equal to that of nums[j].

Return the maximum value of nums[i] + nums[j] that you can obtain over all possible indices i and j that satisfy the conditions.

Example 1:

Input: nums = [18,43,36,13,7]
Output: 54
Explanation: The pairs (i, j) that satisfy the conditions are:
- (0, 2), both numbers have a sum of digits equal to 9, and their sum is 18 + 36 = 54.
- (1, 4), both numbers have a sum of digits equal to 7, and their sum is 43 + 7 = 50.
So the maximum sum that we can obtain is 54.

Example 2:

Input: nums = [10,12,19,14]
Output: -1
Explanation: There are no two numbers that satisfy the conditions, so we return -1.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn digit_sum(a: i32) -> i32 {
            let (mut ret, mut a) = (0, a);
            while a > 0 {
                ret += a % 10;
                a /= 10;
            }
            ret
        }

        let mut mp: HashMap<i32, Vec<i32>> = HashMap::new();
        for a in nums {
            mp.entry(digit_sum(a)).or_default().push(a);
        }
        let mut ret = -1i32;
        for (_, v) in mp {
            if v.len() == 1 {
                continue;
            }
            let mut v = v;
            v.sort();
            ret = ret.max(v[v.len() - 1] + v[v.len() - 2]);
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![(vec![18, 43, 36, 13, 7], 54), (vec![10, 12, 19, 14], -1)];
    for (nums, expect) in cases {
        assert_eq!(Solution::maximum_sum(nums), expect);
    }
}
