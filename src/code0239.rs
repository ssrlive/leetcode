#![allow(dead_code)]

// 239. Sliding Window Maximum
// https://leetcode.com/problems/sliding-window-maximum/
// https://leetcode.cn/problems/sliding-window-maximum/
//
// You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.
//
// Return the max sliding window.
//
// Example 1:
// Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
// Output: [3,3,5,5,6,7]
// Explanation:
// Window position                Max
// ---------------               -----
// [1  3  -1] -3  5  3  6  7       3
//  1 [3  -1  -3] 5  3  6  7       3
//  1  3 [-1  -3  5] 3  6  7       5
//  1  3  -1 [-3  5  3] 6  7       5
//  1  3  -1  -3 [5  3  6] 7       6
//  1  3  -1  -3  5 [3  6  7]      7
//
// Example 2:
// Input: nums = [1], k = 1
// Output: [1]
//
// Example 3:
// Input: nums = [1,-1], k = 1
// Output: [1,-1]
//
// Example 4:
// Input: nums = [9,11], k = 2
// Output: [11]
//
// Example 5:
// Input: nums = [4,-2], k = 2
// Output: [4]
//
// Constraints:
// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4
// 1 <= k <= nums.length
//
// Follow up: Could you solve it in linear time?
//

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut decreasing_window = std::collections::VecDeque::new();
        let mut result = Vec::new();
        for i in 0..nums.len() {
            while let Some(&back_index) = decreasing_window.back() {
                if nums[i] < nums[back_index as usize] {
                    break;
                }
                decreasing_window.pop_back();
            }
            decreasing_window.push_back(i as i32);
            if i as i32 >= k - 1 {
                while let Some(&front_index) = decreasing_window.front() {
                    if front_index > i as i32 - k {
                        break;
                    }
                    decreasing_window.pop_front();
                }
                result.push(nums[*decreasing_window.front().unwrap_or(&0) as usize]);
            }
        }
        result
    }
}

#[test]
fn test_max_sliding_window() {
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
    assert_eq!(Solution::max_sliding_window(vec![9, 11], 2), vec![11]);
    assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]);
}
