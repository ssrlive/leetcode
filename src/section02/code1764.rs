#![allow(dead_code)]

/*

// 1764. Form Array by Concatenating Subarrays of Another Array
// https://leetcode.com/problems/form-array-by-concatenating-subarrays-of-another-array/
// https://leetcode.cn/problems/form-array-by-concatenating-subarrays-of-another-array/
//
// Medium
//
// You are given a 2D integer array groups of length n. You are also given an integer array nums.

You are asked if you can choose n disjoint subarrays from the array nums such that the ith subarray is equal to groups[i] (0-indexed), and if i > 0, the (i-1)th subarray appears before the ith subarray in nums (i.e. the subarrays must be in the same order as groups).

Return true if you can do this task, and false otherwise.

Note that the subarrays are disjoint if and only if there is no index k such that nums[k] belongs to more than one subarray. A subarray is a contiguous sequence of elements within an array.

Example 1:

Input: groups = [[1,-1,-1],[3,-2,0]], nums = [1,-1,0,1,-1,-1,3,-2,0]
Output: true
Explanation: You can choose the 0th subarray as [1,-1,0,1,-1,-1,3,-2,0] and the 1st one as [1,-1,0,1,-1,-1,3,-2,0].
These subarrays are disjoint as they share no common nums[k] element.

Example 2:

Input: groups = [[10,-2],[1,2,3,4]], nums = [1,2,3,4,10,-2]
Output: false
Explanation: Note that choosing the subarrays [1,2,3,4,10,-2] and [1,2,3,4,10,-2] is incorrect because they are not in the same order as in groups.
[10,-2] must come before [1,2,3,4].

Example 3:

Input: groups = [[1,2,3],[3,4]], nums = [7,7,1,2,3,4,7,7]
Output: false
Explanation: Note that choosing the subarrays [7,7,1,2,3,4,7,7] and [7,7,1,2,3,4,7,7] is invalid because they are not disjoint.
They share a common elements nums[4] (0-indexed).

Constraints:

    groups.length == n
    1 <= n <= 10^3
    1 <= groups[i].length, sum(groups[i].length) <= 10^3
    1 <= nums.length <= 10^3
    -10^7 <= groups[i][j], nums[k] <= 10^7
*/

struct Solution;

impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        fn helper(a: &[i32], b: &[i32]) -> bool {
            for i in 0..a.len() {
                if a[i] != b[i] {
                    return false;
                }
            }
            true
        }

        let n = groups.len();
        let m = nums.len();
        let mut i = 0;
        let mut j = 0;

        while i < m {
            if n <= j {
                return true;
            }

            let group = &groups[j];
            let len = group.len();
            if m < i + len {
                return false;
            }

            if helper(group, &nums[i..i + len]) {
                j += 1;
                i += len;
            } else {
                i += 1;
            }
        }

        n <= j
    }
}

#[test]
fn test() {
    let groups = vec![vec![1, -1, -1], vec![3, -2, 0]];
    let nums = vec![1, -1, 0, 1, -1, -1, 3, -2, 0];
    assert!(Solution::can_choose(groups, nums));

    let groups = vec![vec![10, -2], vec![1, 2, 3, 4]];
    let nums = vec![1, 2, 3, 4, 10, -2];
    assert!(!Solution::can_choose(groups, nums));

    let groups = vec![vec![1, 2, 3], vec![3, 4]];
    let nums = vec![7, 7, 1, 2, 3, 4, 7, 7];
    assert!(!Solution::can_choose(groups, nums));
}
