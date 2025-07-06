#![allow(dead_code)]

// 2708. Maximum Strength of a Group
// https://leetcode.com/problems/maximum-strength-of-a-group/
// https://leetcode.cn/problems/maximum-strength-of-a-group/
//
// Medium
//
// You are given a 0-indexed integer array nums representing the score of students in an exam.
// The teacher would like to form one non-empty group of students with maximal strength,
// where the strength of a group of students of indices i0, i1, i2, ... , ik is defined
// as nums[i0] * nums[i1] * nums[i2] * ... * nums[ik​].
//
// Return the maximum strength of a group the teacher can create.
//
// Example 1:
//
// Input: nums = [3,-1,-5,2,5,-9]
// Output: 1350
// Explanation: One way to form a group of maximal strength is to group the students at indices [0,2,3,4,5].
// Their strength is 3 * (-5) * 2 * 5 * (-9) = 1350, which we can show is optimal.
//
// Example 2:
//
// Input: nums = [-4,-5,-4]
// Output: 20
// Explanation: Group the students at indices [0, 1] . Then, we’ll have a resulting strength of 20.
// We cannot achieve greater strength.
//
// Constraints:
//
//     1 <= nums.length <= 13
//     -9 <= nums[i] <= 9
//

struct Solution;

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let (mut positive, mut negative) = (vec![], vec![]);
        let n = nums.len();

        for a in nums {
            if a < 0 {
                negative.push(a as i64);
            }
            if a > 0 {
                positive.push(a as i64);
            }
        }

        if negative.len() == 1 {
            if n == 1 {
                return negative[0];
            }
            if positive.is_empty() {
                return 0;
            }
            let mut ret = 1;
            for a in positive {
                ret *= a;
            }
            return ret;
        }

        if negative.is_empty() && positive.is_empty() {
            return 0;
        }

        let mut ret = 1;
        for a in positive {
            ret *= a;
        }
        negative.sort();
        if !negative.len().is_multiple_of(2) {
            negative.pop();
        }
        for a in negative {
            ret *= a;
        }

        ret
    }
}

#[test]
fn test() {
    let nums = vec![3, -1, -5, 2, 5, -9];
    assert_eq!(Solution::max_strength(nums), 1350);

    let nums = vec![-4, -5, -4];
    assert_eq!(Solution::max_strength(nums), 20);

    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(Solution::max_strength(nums), 5040);
}
