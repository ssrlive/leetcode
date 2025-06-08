#![allow(dead_code)]

// 3576. Transform Array to All Equal Elements
// https://leetcode.com/problems/transform-array-to-all-equal-elements/
// https://leetcode.cn/problems/transform-array-to-all-equal-elements/
//
// Medium
//
// You are given an integer array nums of size n containing only 1 and -1, and an integer k.
//
// You can perform the following operation at most k times:
//
//     Choose an index i (0 <= i < n - 1), and multiply both nums[i] and nums[i + 1] by -1.
//
// Note that you can choose the same index i more than once in different operations.
//
// Return true if it is possible to make all elements of the array equal after at most k operations, and false otherwise.
//
// Example 1:
//
// Input: nums = [1,-1,1,-1,1], k = 3
//
// Output: true
//
// Explanation:
//
// We can make all elements in the array equal in 2 operations as follows:
//
//     Choose index i = 1, and multiply both nums[1] and nums[2] by -1. Now nums = [1,1,-1,-1,1].
//     Choose index i = 2, and multiply both nums[2] and nums[3] by -1. Now nums = [1,1,1,1,1].
//
// Example 2:
//
// Input: nums = [-1,-1,-1,1,1,1], k = 5
//
// Output: false
//
// Explanation:
//
// It is not possible to make all array elements equal in at most 5 operations.
//
// Constraints:
//
//     1 <= n == nums.length <= 10^5
//     nums[i] is either -1 or 1.
//     1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        let mut nums = nums;
        let n = nums.len();
        let mut posistep = 0;
        let mut negistep = 0;
        let mut v = nums.clone();

        for i in 0..n - 1 {
            if v[i] == 1 {
                continue;
            } else {
                v[i] = 1;
                v[i + 1] = -v[i + 1];
                posistep += 1;
            }
        }
        if v[n - 1] == -1 {
            posistep = -1;
        }

        for i in 0..n - 1 {
            if nums[i] == -1 {
                continue;
            } else {
                negistep += 1;
                nums[i] = -1;
                nums[i + 1] = -nums[i + 1];
            }
        }
        if nums[n - 1] == 1 {
            negistep = -1;
        }

        if posistep == -1 && negistep == -1 {
            false
        } else if posistep != -1 && negistep != -1 {
            posistep.min(negistep) <= k
        } else if posistep != -1 {
            posistep <= k
        } else {
            negistep <= k
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, -1, 1, -1, 1];
    let k = 3;
    assert!(Solution::can_make_equal(nums, k));

    let nums = vec![-1, -1, -1, 1, 1, 1];
    let k = 5;
    assert!(!Solution::can_make_equal(nums, k));
}
