#![allow(dead_code)]

// 3159. Find Occurrences of an Element in an Array
// https://leetcode.com/problems/find-occurrences-of-an-element-in-an-array/
// https://leetcode.cn/problems/find-occurrences-of-an-element-in-an-array/
//
// Medium
//
// You are given an integer array nums, an integer array queries, and an integer x.
//
// For each queries[i], you need to find the index of the queries[i]th occurrence of x in the nums array. If there are fewer than queries[i] occurrences of x, the answer should be -1 for that query.
//
// Return an integer array answer containing the answers to all queries.
//
// Example 1:
//
// Input: nums = [1,3,1,7], queries = [1,3,2,4], x = 1
//
// Output: [0,-1,2,-1]
//
// Explanation:
//
// For the 1st query, the first occurrence of 1 is at index 0.
// For the 2nd query, there are only two occurrences of 1 in nums, so the answer is -1.
// For the 3rd query, the second occurrence of 1 is at index 2.
// For the 4th query, there are only two occurrences of 1 in nums, so the answer is -1.
//
// Example 2:
//
// Input: nums = [1,2,3], queries = [10], x = 5
//
// Output: [-1]
//
// Explanation:
//
// For the 1st query, 5 doesn't exist in nums, so the answer is -1.
//
// Constraints:
//
// 1 <= nums.length, queries.length <= 10^5
// 1 <= queries[i] <= 10^5
// 1 <= nums[i], x <= 10^4
//

struct Solution;

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        queries
            .into_iter()
            .scan(
                nums.into_iter()
                    .enumerate()
                    .filter(|(_, i)| *i == x)
                    .map(|(i, _)| i as i32)
                    .collect::<Vec<i32>>(),
                |v, q| Some(*v.get(q as usize - 1).unwrap_or(&(-1))),
            )
            .collect::<_>()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1),
        vec![0, -1, 2, -1]
    );
    assert_eq!(Solution::occurrences_of_element(vec![1, 2, 3], vec![10], 5), vec![-1]);
}
