#![allow(dead_code)]

// 3191. Minimum Operations to Make Binary Array Elements Equal to One I
// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/
// https://leetcode.cn/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/
//
// Medium
//
// You are given a binary array nums.
//
// You can do the following operation on the array any number of times (possibly zero):
//
//     Choose any 3 consecutive elements from the array and flip all of them.
//
// Flipping an element means changing its value from 0 to 1, and from 1 to 0.
//
// Return the minimum number of operations required to make all elements in nums equal to 1. If it is impossible, return -1.
//
// Example 1:
//
// Input: nums = [0,1,1,1,0,0]
//
// Output: 3
//
// Explanation:
// We can do the following operations:
//
//     Choose the elements at indices 0, 1 and 2. The resulting array is nums = [1,0,0,1,0,0].
//     Choose the elements at indices 1, 2 and 3. The resulting array is nums = [1,1,1,0,0,0].
//     Choose the elements at indices 3, 4 and 5. The resulting array is nums = [1,1,1,1,1,1].
//
// Example 2:
//
// Input: nums = [0,1,1,1]
//
// Output: -1
//
// Explanation:
// It is impossible to make all elements equal to 1.
//
// Constraints:
//
//     3 <= nums.length <= 10^5
//     0 <= nums[i] <= 1
//

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut res = 0;
        for i in 0..nums.len() - 2 {
            if nums[i] == 0 {
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                res += 1;
            }
        }
        if nums[nums.len() - 2] == 0 || nums[nums.len() - 1] == 0 {
            return -1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 1]), -1);
    assert_eq!(Solution::min_operations(vec![1, 1, 1, 1]), 0);
}
