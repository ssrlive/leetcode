#![allow(dead_code)]

// 3362. Zero Array Transformation III
// https://leetcode.com/problems/zero-array-transformation-iii/
// https://leetcode.cn/problems/zero-array-transformation-iii/
//
// Medium
//
// You are given an integer array nums of length n and a 2D array queries where queries[i] = [li, ri].
//
// Each queries[i] represents the following action on nums:
//
// Decrement the value at each index in the range [li, ri] in nums by at most 1.
// The amount by which the value is decremented can be chosen independently for each index.
// A Zero Array is an array with all its elements equal to 0.
//
// Return the maximum number of elements that can be removed from queries, such that nums can still be converted to a zero array using the remaining queries. If it is not possible to convert nums to a zero array, return -1.
//
// Example 1:
//
// Input: nums = [2,0,2], queries = [[0,2],[0,2],[1,1]]
//
// Output: 1
//
// Explanation:
//
// After removing queries[2], nums can still be converted to a zero array.
//
// Using queries[0], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
// Using queries[1], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
//
// Example 2:
//
// Input: nums = [1,1,1,1], queries = [[1,3],[0,2],[1,3],[1,2]]
//
// Output: 2
//
// Explanation:
//
// We can remove queries[2] and queries[3].
//
// Example 3:
//
// Input: nums = [1,2,3,4], queries = [[0,3]]
//
// Output: -1
//
// Explanation:
//
// nums cannot be converted to a zero array even after using all the queries.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 0 <= nums[i] <= 10^5
// 1 <= queries.length <= 10^5
// queries[i].length == 2
// 0 <= li <= ri < nums.length
//

struct Solution;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        let mut queries = queries;
        queries.sort();
        let mut candidate = std::collections::BinaryHeap::new();
        let mut chosen = std::collections::BinaryHeap::new();
        let mut ans = 0;
        let n = nums.len() as i32;
        let mut j = 0;
        for i in 0..n {
            while j < queries.len() as i32 && queries[j as usize][0] == i {
                candidate.push(queries[j as usize][1]);
                j += 1;
            }
            nums[i as usize] -= chosen.len() as i32;
            while nums[i as usize] > 0 && !candidate.is_empty() && candidate.peek().unwrap() >= &i {
                ans += 1;
                chosen.push(std::cmp::Reverse(candidate.pop().unwrap()));
                nums[i as usize] -= 1;
            }
            if nums[i as usize] > 0 {
                return -1;
            }
            while !chosen.is_empty() && chosen.peek().unwrap() == &std::cmp::Reverse(i) {
                chosen.pop();
            }
        }
        queries.len() as i32 - ans
    }
}

#[test]
fn test() {
    let nums = vec![2, 0, 2];
    let queries = vec![vec![0, 2], vec![0, 2], vec![1, 1]];
    let res = 1;
    assert_eq!(Solution::max_removal(nums, queries), res);

    let nums = vec![1, 1, 1, 1];
    let queries = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]];
    let res = 2;
    assert_eq!(Solution::max_removal(nums, queries), res);

    let nums = vec![1, 2, 3, 4];
    let queries = vec![vec![0, 3]];
    let res = -1;
    assert_eq!(Solution::max_removal(nums, queries), res);

    let nums = vec![4, 5];
    let queries = vec![vec![1, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![1, 1], vec![0, 0]];
    let res = 0;
    assert_eq!(Solution::max_removal(nums, queries), res);
}
