#![allow(dead_code)]

/*

// 1509. Minimum Difference Between Largest and Smallest Value in Three Moves
// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/
// https://leetcode.cn/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/
//
// Medium
//
// You are given an integer array nums.
//
// In one move, you can choose one element of nums and change it to any value.
//
// Return the minimum difference between the largest and smallest value of nums after performing at most three moves.

Example 1:

Input: nums = [5,3,2,4]
Output: 0
Explanation: We can make at most 3 moves.
In the first move, change 2 to 3. nums becomes [5,3,3,4].
In the second move, change 4 to 3. nums becomes [5,3,3,3].
In the third move, change 5 to 3. nums becomes [3,3,3,3].
After performing 3 moves, the difference between the minimum and maximum is 3 - 3 = 0.

Example 2:

Input: nums = [1,5,0,10,14]
Output: 1
Explanation: We can make at most 3 moves.
In the first move, change 5 to 0. nums becomes [1,0,0,10,14].
In the second move, change 10 to 0. nums becomes [1,0,0,0,14].
In the third move, change 14 to 1. nums becomes [1,0,0,0,1].
After performing 3 moves, the difference between the minimum and maximum is 1 - 0 = 0.
It can be shown that there is no way to make the difference 0 in 3 moves.

Example 3:

Input: nums = [3,100,20]
Output: 0
Explanation: We can make at most 3 moves.
In the first move, change 100 to 7. nums becomes [4,7,20].
In the second move, change 20 to 7. nums becomes [4,7,7].
In the third move, change 4 to 3. nums becomes [7,7,7].
After performing 3 moves, the difference between the minimum and maximum is 7 - 7 = 0.

Constraints:

    1 <= nums.length <= 10^5
    -10^9 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut min = i32::MAX;
        for l in 0..4 {
            let r = nums.len().saturating_sub(4 - l);
            if l >= r {
                return 0;
            }
            min = min.min(nums[r] - nums[l]);
        }
        min
    }
}

#[test]
fn test() {
    let cases = vec![(vec![5, 3, 2, 4], 0), (vec![1, 5, 0, 10, 14], 1), (vec![3, 100, 20], 0)];
    for (nums, expected) in cases {
        assert_eq!(Solution::min_difference(nums), expected);
    }
}
