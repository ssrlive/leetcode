#![allow(dead_code)]

// 18. 4Sum
// https://leetcode.com/problems/4sum/
// https://leetcode.cn/problems/4sum/
//
// Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
//
// 0 <= a, b, c, d < n
// a, b, c, and d are distinct.
// nums[a] + nums[b] + nums[c] + nums[d] == target
// You may return the answer in any order.
//
// Example 1:
//
// Input: nums = [1,0,-1,0,-2,2], target = 0
// Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//
// Example 2:
//
// Input: nums = [2,2,2,2,2], target = 8
// Output: [[2,2,2,2]]
//
// Constraints:
//
// - 1 <= nums.length <= 200
// - -10^9 <= nums[i] <= 10^9
// - -10^9 <= target <= 10^9
//

struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        nums.sort();
        let mut ret = vec![];
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            Self::three_sum(nums[i], &nums[i + 1..nums.len()], target, &mut ret);
        }
        ret
    }

    fn three_sum(first: i32, nums: &[i32], target: i32, ret: &mut Vec<Vec<i32>>) {
        let target = target as i64 - first as i64;
        for i in 0..nums.len() - 2 {
            let a = nums[i];
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            while l < r {
                let (b, c) = (nums[l], nums[r]);
                let sum = a as i64 + b as i64 + c as i64;
                match sum.cmp(&target) {
                    std::cmp::Ordering::Less => l += 1,
                    std::cmp::Ordering::Greater => r -= 1,
                    std::cmp::Ordering::Equal => {
                        ret.push(vec![first, a, b, c]);
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1
                        }
                        r -= 1;
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1
                        }
                        l += 1;
                    }
                }
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let mut ret = Solution::four_sum(nums, target);
    ret.sort();
    let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
    assert_eq!(ret, expected);

    let nums = vec![2, 2, 2, 2, 2];
    let target = 8;
    let mut ret = Solution::four_sum(nums, target);
    ret.sort();
    let expected = vec![vec![2, 2, 2, 2]];
    assert_eq!(ret, expected);
}
