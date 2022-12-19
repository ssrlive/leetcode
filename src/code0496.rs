#![allow(dead_code)]

// 496. Next Greater Element I
// https://leetcode.com/problems/next-greater-element-i/
// https://leetcode.cn/problems/next-greater-element-i/
//
// The next greater element of some element x in an array is the first greater element that is to the right of x in the same array.
//
// You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of nums2.
//
// For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2. If there is no next greater element, then the answer for this query is -1.
//
// Return an array ans of length nums1.length such that ans[i] is the next greater element as described above.
//
// Example 1:
//
// Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
// Output: [-1,3,-1]
// Explanation: The next greater element for each value of nums1 is as follows:
// - 4 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
// - 1 is underlined in nums2 = [1,3,4,2]. The next greater element is 3.
// - 2 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
//
// Example 2:
//
// Input: nums1 = [2,4], nums2 = [1,2,3,4]
// Output: [3,-1]
// Explanation: The next greater element for each value of nums1 is as follows:
// - 2 is underlined in nums2 = [1,2,3,4]. The next greater element is 3.
// - 4 is underlined in nums2 = [1,2,3,4]. There is no next greater element, so the answer is -1.
//
// Constraints:
//
// - 1 <= nums1.length <= nums2.length <= 1000
// - 0 <= nums1[i], nums2[i] <= 10^4
// - All integers in nums1 and nums2 are unique.
// - All the integers of nums1 also appear in nums2.
//
// Follow up: Could you find an O(nums1.length + nums2.length) solution?
//

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut next_greater = vec![-1; nums2.len()];
        let mut stack = Vec::new();
        for (i, &num) in nums2.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if nums2[top] < num {
                    next_greater[top] = num;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        nums1
            .into_iter()
            .map(|num| next_greater[nums2.iter().position(|&n| n == num).unwrap()])
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}
