#![allow(dead_code)]

// 239. Sliding Window Maximum
// https://leetcode.com/problems/sliding-window-maximum/
// https://leetcode.cn/problems/sliding-window-maximum/
// https://leetcode.cn/problems/sliding-window-maximum/solution/by-liu-nian-9r-6l90/
//
// Medium
//
// You are given an array of integers nums, there is a sliding window of size k which is moving from the very
// left of the array to the very right. You can only see the k numbers in the window.
// Each time the sliding window moves right by one position.
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
        fn _max_sliding_window(nums: Vec<i32>, k: i32) -> Option<Vec<i32>> {
            let k = k as usize;
            let mut result = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            for (idx, &num) in nums.iter().enumerate() {
                while !queue.is_empty() && nums[*queue.back()?] <= num {
                    queue.pop_back();
                }
                queue.push_back(idx);
                if *queue.front()? + k <= idx {
                    queue.pop_front();
                }
                if k - 1 <= idx {
                    result.push(nums[*queue.front()?]);
                }
            }
            Some(result)
        }
        _max_sliding_window(nums, k).unwrap_or_default()
    }

    pub fn max_sliding_window_3(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let count = nums.len() - k + 1;
        let mut result = Vec::with_capacity(count);
        let mut queue = std::collections::VecDeque::with_capacity(k);
        nums.iter()
            .take(k)
            .map(|&x| {
                queue.push_back(x);
            })
            .count();
        for i in 0..count {
            if i > 0 {
                queue.pop_front();
                queue.push_back(nums[i + k - 1]);
            }
            let &max = queue.iter().max().unwrap();
            result.push(max);
        }
        result
    }

    pub fn max_sliding_window_2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let count = nums.len() - k + 1;
        let mut result = Vec::with_capacity(count);
        let iter = nums.iter();
        for i in 0..count {
            let &max = iter.clone().skip(i).take(k).max().unwrap();
            result.push(max);
        }
        result
    }
}

#[test]
fn test_max_sliding_window() {
    let cases = vec![
        (vec![1, 3, -1, -3, 5, 3, 6, 7], 3, vec![3, 3, 5, 5, 6, 7]),
        (vec![1], 1, vec![1]),
        (vec![1, -1], 1, vec![1, -1]),
        (vec![9, 11], 2, vec![11]),
        (vec![4, -2], 2, vec![4]),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }
}
