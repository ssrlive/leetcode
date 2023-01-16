#![allow(dead_code)]

// 996. Number of Squareful Arrays
// https://leetcode.com/problems/number-of-squareful-arrays/
// https://leetcode.cn/problems/number-of-squareful-arrays/
//
// An array is squareful if the sum of every pair of adjacent elements is a perfect square.
//
// Given an integer array nums, return the number of permutations of nums that are squareful.
//
// Two permutations perm1 and perm2 are different if there is some index i such that perm1[i] != perm2[i].
//
// Example 1:
//
// Input: nums = [1,17,8]
// Output: 2
// Explanation: [1,8,17] and [17,8,1] are the valid permutations.
//
// Example 2:
//
// Input: nums = [2,2,2]
// Output: 1
//
// Constraints:
//
// - 1 <= nums.length <= 12
// - 0 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;

        fn pmt(nums: Vec<i32>, idx: usize, ans: &mut i32) {
            let mut nums = nums;
            if idx >= nums.len() {
                *ans += 1;
            }
            for i in idx..nums.len() {
                if i > idx && nums[i] == nums[idx] {
                    continue;
                }
                nums.swap(i, idx);
                if idx == 0 || (idx > 0 && is_square(nums[idx] + nums[idx - 1])) {
                    pmt(nums.clone(), idx + 1, ans);
                }
            }
        }

        fn is_square(v: i32) -> bool {
            let r = (v as f64).sqrt() as i32;
            r * r == v
        }

        pmt(nums, 0, &mut ans);
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_squareful_perms(vec![1, 17, 8]), 2);
    assert_eq!(Solution::num_squareful_perms(vec![2, 2, 2]), 1);
}
