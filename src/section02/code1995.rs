#![allow(dead_code)]

/*

// 1995. Count Special Quadruplets
// https://leetcode.com/problems/count-special-quadruplets/
// https://leetcode.cn/problems/count-special-quadruplets/
//
// Easy
//
// Given a 0-indexed integer array nums, return the number of distinct quadruplets (a, b, c, d) such that:

    nums[a] + nums[b] + nums[c] == nums[d], and
    a < b < c < d

Example 1:

Input: nums = [1,2,3,6]
Output: 1
Explanation: The only quadruplet that satisfies the requirement is (0, 1, 2, 3) because 1 + 2 + 3 == 6.

Example 2:

Input: nums = [3,3,6,4,5]
Output: 0
Explanation: There are no such quadruplets in [3,3,6,4,5].

Example 3:

Input: nums = [1,1,1,3,5]
Output: 4
Explanation: The 4 quadruplets that satisfy the requirement are:
- (0, 1, 2, 3): 1 + 1 + 1 == 3
- (0, 1, 3, 4): 1 + 1 + 3 == 5
- (0, 2, 3, 4): 1 + 1 + 3 == 5
- (1, 2, 3, 4): 1 + 1 + 3 == 5

Constraints:

    4 <= nums.length <= 50
    1 <= nums[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut res = 0;
        for (ia, a) in nums[..l - 3].iter().enumerate() {
            for (ib, b) in nums[..l - 2].iter().enumerate().skip(ia + 1) {
                for (ic, c) in nums[..l - 1].iter().enumerate().skip(ib + 1) {
                    let s = a + b + c;
                    for &d in &nums[ic + 1..] {
                        if d == s {
                            res += 1
                        }
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 6]), 1);
    assert_eq!(Solution::count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    assert_eq!(Solution::count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
}
