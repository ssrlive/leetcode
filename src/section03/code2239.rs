#![allow(dead_code)]

/*

// 2239. Find Closest Number to Zero
// https://leetcode.com/problems/find-closest-number-to-zero/
// https://leetcode.cn/problems/find-closest-number-to-zero/
//
// Easy
//
// Given an integer array nums of size n, return the number with the value closest to 0 in nums.
// If there are multiple answers, return the number with the largest value.

Example 1:

Input: nums = [-4,-2,1,4,8]
Output: 1
Explanation:
The distance from -4 to 0 is |-4| = 4.
The distance from -2 to 0 is |-2| = 2.
The distance from 1 to 0 is |1| = 1.
The distance from 4 to 0 is |4| = 4.
The distance from 8 to 0 is |8| = 8.
Thus, the closest number to 0 in the array is 1.

Example 2:

Input: nums = [2,-1,1]
Output: 1
Explanation: 1 and -1 are both the closest numbers to 0, so 1 being larger is returned.

Constraints:

    1 <= n <= 1000
    -10^5 <= nums[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        nums.iter()
            .fold((i32::MIN, i32::MAX), |(val, dist), &x| match dist.cmp(&(x.abs())) {
                Ordering::Greater => (x, x.abs()),
                Ordering::Equal => (x.max(val), dist),
                Ordering::Less => (val, dist),
            })
            .0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
    assert_eq!(Solution::find_closest_number(vec![2, -1, 1]), 1);
}
