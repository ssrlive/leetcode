#![allow(dead_code)]

/*

// 2341. Maximum Number of Pairs in Array
// https://leetcode.com/problems/maximum-number-of-pairs-in-array/
// https://leetcode.cn/problems/maximum-number-of-pairs-in-array/
//
// Easy
//
// You are given a 0-indexed integer array nums. In one operation, you may do the following:

    Choose two integers in nums that are equal.
    Remove both integers from nums, forming a pair.

The operation is done on nums as many times as possible.

Return a 0-indexed integer array answer of size 2 where answer[0] is the number of pairs that are formed and answer[1] is the number of leftover integers in nums after doing the operation as many times as possible.

Example 1:

Input: nums = [1,3,2,1,3,2,2]
Output: [3,1]
Explanation:
Form a pair with nums[0] and nums[3] and remove them from nums. Now, nums = [3,2,3,2,2].
Form a pair with nums[0] and nums[2] and remove them from nums. Now, nums = [2,2,2].
Form a pair with nums[0] and nums[1] and remove them from nums. Now, nums = [2].
No more pairs can be formed. A total of 3 pairs have been formed, and there is 1 number leftover in nums.

Example 2:

Input: nums = [1,1]
Output: [1,0]
Explanation: Form a pair with nums[0] and nums[1] and remove them from nums. Now, nums = [].
No more pairs can be formed. A total of 1 pair has been formed, and there are 0 numbers leftover in nums.

Example 3:

Input: nums = [0]
Output: [0,1]
Explanation: No pairs can be formed, and there is 1 number leftover in nums.

Constraints:

    1 <= nums.length <= 100
    0 <= nums[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut counter = [0; 101];
        nums.iter().for_each(|&x| counter[x as usize] += 1);
        counter.into_iter().fold(vec![0, 0], |mut acc, x| {
            acc[0] += x / 2;
            acc[1] += x % 2;
            acc
        })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]), vec![3, 1]);
    assert_eq!(Solution::number_of_pairs(vec![1, 1]), vec![1, 0]);
    assert_eq!(Solution::number_of_pairs(vec![0]), vec![0, 1]);
}
