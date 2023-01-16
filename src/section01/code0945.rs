#![allow(dead_code)]

// 945. Minimum Increment to Make Array Unique
// https://leetcode.com/problems/minimum-increment-to-make-array-unique/
// https://leetcode.cn/problems/minimum-increment-to-make-array-unique/
//
// You are given an integer array nums. In one move, you can pick an index i where 0 <= i < nums.length and increment nums[i] by 1.
//
// Return the minimum number of moves to make every value in nums unique.
//
// The test cases are generated so that the answer fits in a 32-bit integer.
//
// Example 1:
//
// Input: nums = [1,2,2]
// Output: 1
// Explanation: After 1 move, the array could be [1, 2, 3].
//
// Example 2:
//
// Input: nums = [3,2,1,2,1,7]
// Output: 6
// Explanation: After 6 moves, the array could be [3, 4, 1, 2, 5, 7].
// It can be shown with 5 or less moves that it is impossible for the array to have all unique values.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 0 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut mincount = 0;
        for i in 1..nums.len() {
            match nums[i].cmp(&nums[i - 1]) {
                std::cmp::Ordering::Equal => {
                    nums[i] += 1;
                    mincount += 1;
                }
                std::cmp::Ordering::Less => {
                    let diff = nums[i - 1] - nums[i];
                    nums[i] += diff + 1;
                    mincount += diff + 1;
                }
                _ => {}
            }
        }
        mincount
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
    assert_eq!(Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
}
