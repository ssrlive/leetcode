#![allow(dead_code)]

// 659. Split Array into Consecutive Subsequences
// https://leetcode.com/problems/split-array-into-consecutive-subsequences/
// https://leetcode.cn/problems/split-array-into-consecutive-subsequences/
//
// You are given an integer array nums that is sorted in non-decreasing order.
//
// Determine if it is possible to split nums into one or more subsequences such that both of the following conditions are true:
//
// - Each subsequence is a consecutive increasing sequence (i.e. each integer is exactly one more than the previous integer).
// - All subsequences have a length of 3 or more.
//
// Return true if you can split nums according to the above conditions, or false otherwise.
//
// A subsequence of an array is a new array that is formed from the original array by deleting some (can be none)
// of the elements without disturbing the relative positions of the remaining elements.
// (i.e., [1,3,5] is a subsequence of [1,2,3,4,5] while [1,3,2] is not).
//
// Example 1:
//
// Input: nums = [1,2,3,3,4,5]
// Output: true
// Explanation: nums can be split into the following subsequences:
// [1,2,3,3,4,5] --> 1, 2, 3
// [1,2,3,3,4,5] --> 3, 4, 5
//
// Example 2:
//
// Input: nums = [1,2,3,3,4,4,5,5]
// Output: true
// Explanation: nums can be split into the following subsequences:
// [1,2,3,3,4,4,5,5] --> 1, 2, 3, 4, 5
// [1,2,3,3,4,4,5,5] --> 3, 4, 5
//
// Example 3:
//
// Input: nums = [1,2,3,4,4,5]
// Output: false
// Explanation: It is impossible to split nums into consecutive increasing subsequences of length 3 or more.
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -1000 <= nums[i] <= 1000
// - nums is sorted in non-decreasing order.
//

struct Solution;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::<Reverse<(i32, usize)>>::new();
        for num in nums {
            loop {
                match pq.peek().copied() {
                    None => {
                        pq.push(Reverse((num, 1)));
                        break;
                    }
                    Some(Reverse((head, cnt))) if head + 1 == num => {
                        pq.pop();
                        pq.push(Reverse((num, cnt + 1)));
                        break;
                    }
                    Some(Reverse((head, _))) if head == num => {
                        pq.push(Reverse((num, 1)));
                        break;
                    }
                    Some(Reverse((_, cnt))) => {
                        if cnt < 3 {
                            return false;
                        } else {
                            pq.pop();
                        }
                    }
                }
            }
        }
        pq.into_iter().all(|Reverse((_, cnt))| cnt >= 3)
    }
}

#[test]
fn test() {
    assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
    assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
    assert!(!Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
}
