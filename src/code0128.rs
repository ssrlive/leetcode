#![allow(dead_code)]

// 128. Longest Consecutive Sequence
// https://leetcode.com/problems/longest-consecutive-sequence/
//
// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
//
// Follow up: Could you implement the O(n) solution?

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut max_len = 0;
        let mut cur_len = 0;
        let mut prev = std::i32::MIN;
        for n in nums {
            if n == prev {
                continue;
            }

            if n == prev + 1 {
                cur_len += 1;
            } else {
                max_len = max_len.max(cur_len);
                cur_len = 1;
            }

            prev = n;
        }

        max_len.max(cur_len)
    }
}

#[test]
fn test_longest_consecutive() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
}
