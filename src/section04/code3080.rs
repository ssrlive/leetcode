#![allow(dead_code)]

// 3080. Mark Elements on Array by Performing Queries
// https://leetcode.com/problems/mark-elements-on-array-by-performing-queries/
// https://leetcode.cn/problems/mark-elements-on-array-by-performing-queries/
//
// Medium
//
// You are given a 0-indexed array nums of size n consisting of positive integers.
//
// You are also given a 2D array queries of size m where queries[i] = [indexi, ki].
//
// Initially all elements of the array are unmarked.
//
// You need to apply m queries on the array in order, where on the ith query you do the following:
//
//     Mark the element at index indexi if it is not already marked.
//     Then mark ki unmarked elements in the array with the smallest values. If multiple such elements exist,
//     mark the ones with the smallest indices. And if less than ki unmarked elements exist, then mark all of them.
//
// Return an array answer of size m where answer[i] is the sum of unmarked elements in the array after the ith query.
//
// Example 1:
//
// Input: nums = [1,2,2,1,2,3,1], queries = [[1,2],[3,3],[4,2]]
//
// Output: [8,3,0]
//
// Explanation:
//
// We do the following queries on the array:
//
// - Mark the element at index 1, and 2 of the smallest unmarked elements with the smallest indices if they exist,
//   the marked elements now are nums = [1,2,2,1,2,3,1]. The sum of unmarked elements is 2 + 2 + 3 + 1 = 8.
// - Mark the element at index 3, since it is already marked we skip it. Then we mark 3 of the smallest unmarked elements
//   with the smallest indices, the marked elements now are nums = [1,2,2,1,2,3,1]. The sum of unmarked elements is 3.
// - Mark the element at index 4, since it is already marked we skip it. Then we mark 2 of the smallest unmarked elements
//   with the smallest indices if they exist, the marked elements now are nums = [1,2,2,1,2,3,1]. The sum of unmarked elements is 0.
//
// Example 2:
//
// Input: nums = [1,4,2,3], queries = [[0,1]]
//
// Output: [7]
//
// Explanation: We do one query which is mark the element at index 0 and mark the smallest element among unmarked elements.
// The marked elements will be nums = [1,4,2,3], and the sum of unmarked elements is 4 + 3 = 7.
//
// Constraints:
//
//     n == nums.length
//     m == queries.length
//     1 <= m <= n <= 10^5
//     1 <= nums[i] <= 10^5
//     queries[i].length == 2
//     0 <= indexi, ki <= n - 1
//

struct Solution;

impl Solution {
    pub fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums.len();
        let mut data = Vec::with_capacity(n);
        let mut total = 0;
        for (i, &nums_i) in nums.iter().enumerate().take(n) {
            data.push((nums_i, i));
            total += nums_i as i64;
        }
        data.sort_unstable();

        let mut marked = vec![false; n];
        let (mut res, mut pos) = (Vec::with_capacity(queries.len()), 0);

        for q in queries {
            let (p, mut k) = (q[0] as usize, q[1]);
            if !marked[p] {
                marked[p] = true;
                total -= nums[p] as i64;
            }

            while pos < n && k > 0 {
                if !marked[data[pos].1] {
                    total -= data[pos].0 as i64;
                    marked[data[pos].1] = true;
                    k -= 1;
                }
                pos += 1;
            }

            res.push(total);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::unmarked_sum_array(vec![1, 2, 2, 1, 2, 3, 1], vec![vec![1, 2], vec![3, 3], vec![4, 2]]),
        vec![8, 3, 0]
    );
    assert_eq!(Solution::unmarked_sum_array(vec![1, 4, 2, 3], vec![vec![0, 1]]), vec![7]);
}
