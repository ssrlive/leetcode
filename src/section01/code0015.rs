#![allow(dead_code)]

// 15. 3Sum
// https://leetcode.com/problems/3sum/
// https://leetcode.cn/problems/3sum/
//
// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//
// Notice that the solution set must not contain duplicate triplets.
//
// Example 1:
//
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.
//
// Example 2:
//
// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.
//
// Example 3:
//
// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.
//
// Constraints:
//
// - 3 <= nums.length <= 3000
// - -10^5 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result = vec![];
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                    std::cmp::Ordering::Less => {
                        left += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        right -= 1;
                    }
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(nums);
    assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    let nums = vec![0, 1, 1];
    let result = Solution::three_sum(nums);
    assert_eq!(result, Vec::<Vec<i32>>::new());

    let nums = vec![0, 0, 0];
    let result = Solution::three_sum(nums);
    assert_eq!(result, vec![vec![0, 0, 0]]);
}
