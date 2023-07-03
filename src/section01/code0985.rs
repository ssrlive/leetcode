#![allow(dead_code)]

// 985. Sum of Even Numbers After Queries
// https://leetcode.com/problems/sum-of-even-numbers-after-queries/
// https://leetcode.cn/problems/sum-of-even-numbers-after-queries/
//
// You are given an integer array nums and an array queries where queries[i] = [vali, indexi].
//
// For each query i, first, apply nums[indexi] = nums[indexi] + vali, then print the sum of the even values of nums.
//
// Return an integer array answer where answer[i] is the answer to the ith query.
//
// Example 1:
//
// Input: nums = [1,2,3,4], queries = [[1,0],[-3,1],[-4,0],[2,3]]
// Output: [8,6,2,4]
// Explanation: At the beginning, the array is [1,2,3,4].
// After adding 1 to nums[0], the array is [2,2,3,4], and the sum of even values is 2 + 2 + 4 = 8.
// After adding -3 to nums[1], the array is [2,-1,3,4], and the sum of even values is 2 + 4 = 6.
// After adding -4 to nums[0], the array is [-2,-1,3,4], and the sum of even values is -2 + 4 = 2.
// After adding 2 to nums[3], the array is [-2,-1,3,6], and the sum of even values is -2 + 6 = 4.
//
// Example 2:
//
// Input: nums = [1], queries = [[4,0]]
// Output: [0]
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^4 <= nums[i] <= 10^4
// - 1 <= queries.length <= 10^4
// - -10^4 <= vali <= 10^4
// - 0 <= indexi < nums.length
//

struct Solution;

impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        queries
            .iter()
            .filter_map(|query| {
                if let &[val, index] = query.as_slice() {
                    Some((val, index as usize))
                } else {
                    None
                }
            })
            .scan(nums.iter().filter(|num| **num % 2 == 0).sum::<i32>(), |sum, (val, index)| {
                if nums[index] % 2 == 0 {
                    *sum -= nums[index]
                }
                nums[index] += val;
                if nums[index] % 2 == 0 {
                    *sum += nums[index]
                }
                Some(*sum)
            })
            .collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]],
            vec![8, 6, 2, 4],
        ),
        (vec![1], vec![vec![4, 0]], vec![0]),
    ];
    for (nums, queries, expected) in cases {
        assert_eq!(Solution::sum_even_after_queries(nums, queries), expected);
    }
}
