#![allow(dead_code)]

// 480. Sliding Window Median
// https://leetcode.com/problems/sliding-window-median/
// https://leetcode.cn/problems/sliding-window-median/
//
// he median is the middle value in an ordered integer list. If the size of the list is even,
// there is no middle value. So the median is the mean of the two middle values.
//
// For examples, if arr = [2,3,4], the median is 3.
// For examples, if arr = [1,2,3,4], the median is (2 + 3) / 2 = 2.5.
// You are given an integer array nums and an integer k. There is a sliding window of
// size k which is moving from the very left of the array to the very right.
// You can only see the k numbers in the window. Each time the sliding window moves right by one position.
//
// Return the median array for each window in the original array. Answers within 10-5 of the actual value will be accepted.
//
// Example 1:
//
// Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
// Output: [1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000]
// Explanation:
// Window position                Median
// ---------------                -----
// [1  3  -1] -3  5  3  6  7        1
//  1 [3  -1  -3] 5  3  6  7       -1
//  1  3 [-1  -3  5] 3  6  7       -1
//  1  3  -1 [-3  5  3] 6  7        3
//  1  3  -1  -3 [5  3  6] 7        5
//  1  3  -1  -3  5 [3  6  7]       6
//
// Example 2:
//
// Input: nums = [1,2,3,4,2,3,1,4,2], k = 3
// Output: [2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000]
//
// Constraints:
//
// - 1 <= k <= nums.length <= 10^5
// - -2^31 <= nums[i] <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut ans = vec![];
        let mut m = Vec::new();
        let k = k as usize;
        if nums.len() < k {
            return ans;
        }
        for &item in nums.iter().take(k) {
            m.push(item);
        }
        m.sort();
        ans.push(Self::find_median(&mut m, 0, 0));
        for i in k..nums.len() {
            ans.push(Self::find_median(&mut m, nums[i - k], nums[i]));
        }
        ans
    }

    fn find_median(m: &mut Vec<i32>, remove: i32, add: i32) -> f64 {
        if remove != add {
            let pos = m.binary_search(&add).unwrap_or_else(|e| e);
            m.insert(pos, add);
            if let Ok(pos) = m.binary_search(&remove) {
                m.remove(pos);
            }
        }
        let n = m.len();
        if n & 1 == 1 {
            m[n / 2] as f64
        } else {
            (m[n / 2 - 1] as f64 + m[n / 2] as f64) / 2.0
        }
    }
}

#[test]
fn test_median_sliding_window() {
    assert_eq!(
        Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0]
    );
    assert_eq!(
        Solution::median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3),
        vec![2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0]
    );
    assert_eq!(Solution::median_sliding_window(vec![1], 1), vec![1.0]);
    assert_eq!(
        Solution::median_sliding_window(vec![i32::MAX, i32::MAX], 2),
        vec![2147483647.0]
    );
}
