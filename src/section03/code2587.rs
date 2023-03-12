#![allow(dead_code)]

/*

// 2587. Rearrange Array to Maximize Prefix Score
// https://leetcode.com/problems/rearrange-array-to-maximize-prefix-score/
// https://leetcode.cn/problems/rearrange-array-to-maximize-prefix-score/
//
// Medium
//
// You are given a 0-indexed integer array nums. You can rearrange the elements of nums to any order (including the given order).

Let prefix be the array containing the prefix sums of nums after rearranging it. In other words, prefix[i] is the sum of the elements from 0 to i in nums after rearranging it. The score of nums is the number of positive integers in the array prefix.

Return the maximum score you can achieve.

Example 1:

Input: nums = [2,-1,0,1,-3,3,-3]
Output: 6
Explanation: We can rearrange the array into nums = [2,3,1,-1,-3,0,-3].
prefix = [2,5,6,5,2,2,-1], so the score is 6.
It can be shown that 6 is the maximum score we can obtain.

Example 2:

Input: nums = [-2,-3,0]
Output: 0
Explanation: Any rearrangement of the array will result in a score of 0.

Constraints:

    1 <= nums.length <= 10^5
    -10^6 <= nums[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a));
        let mut sum = 0;
        for i in 0..=nums.len() {
            if i == nums.len() {
                return i as i32;
            }
            sum += nums[i] as i64;
            if sum <= 0 {
                return i as i32;
            }
        }
        0
    }
}

#[test]
fn test() {
    let nums = vec![2, -1, 0, 1, -3, 3, -3];
    let ans = 6;
    assert_eq!(Solution::max_score(nums), ans);
    let nums = vec![-2, -3, 0];
    let ans = 0;
    assert_eq!(Solution::max_score(nums), ans);
    let nums = vec![
        -687767, -860350, 950296, 52109, 510127, 545329, -291223, -966728, 852403, 828596, 456730, -483632, -529386,
        356766, 147293, 572374, 243605, 931468, 641668, 494446,
    ];
    let ans = 20;
    assert_eq!(Solution::max_score(nums), ans);
}
