#![allow(dead_code)]

/*

// 2382. Maximum Segment Sum After Removals
// https://leetcode.com/problems/maximum-segment-sum-after-removals/
// https://leetcode.cn/problems/maximum-segment-sum-after-removals/
//
// Hard
//
// You are given two 0-indexed integer arrays nums and removeQueries, both of length n. For the ith query, the element in nums at the index removeQueries[i] is removed, splitting nums into different segments.

A segment is a contiguous sequence of positive integers in nums. A segment sum is the sum of every element in a segment.

Return an integer array answer, of length n, where answer[i] is the maximum segment sum after applying the ith removal.

Note: The same index will not be removed more than once.

Example 1:

Input: nums = [1,2,5,6,1], removeQueries = [0,3,2,4,1]
Output: [14,7,2,2,0]
Explanation: Using 0 to indicate a removed element, the answer is as follows:
Query 1: Remove the 0th element, nums becomes [0,2,5,6,1] and the maximum segment sum is 14 for segment [2,5,6,1].
Query 2: Remove the 3rd element, nums becomes [0,2,5,0,1] and the maximum segment sum is 7 for segment [2,5].
Query 3: Remove the 2nd element, nums becomes [0,2,0,0,1] and the maximum segment sum is 2 for segment [2].
Query 4: Remove the 4th element, nums becomes [0,2,0,0,0] and the maximum segment sum is 2 for segment [2].
Query 5: Remove the 1st element, nums becomes [0,0,0,0,0] and the maximum segment sum is 0, since there are no segments.
Finally, we return [14,7,2,2,0].

Example 2:

Input: nums = [3,2,11,1], removeQueries = [3,2,1,0]
Output: [16,5,3,0]
Explanation: Using 0 to indicate a removed element, the answer is as follows:
Query 1: Remove the 3rd element, nums becomes [3,2,11,0] and the maximum segment sum is 16 for segment [3,2,11].
Query 2: Remove the 2nd element, nums becomes [3,2,0,0] and the maximum segment sum is 5 for segment [3,2].
Query 3: Remove the 1st element, nums becomes [3,0,0,0] and the maximum segment sum is 3 for segment [3].
Query 4: Remove the 0th element, nums becomes [0,0,0,0] and the maximum segment sum is 0, since there are no segments.
Finally, we return [16,5,3,0].

Constraints:

    n == nums.length == removeQueries.length
    1 <= n <= 10^5
    1 <= nums[i] <= 10^9
    0 <= removeQueries[i] < n
    All the values of removeQueries are unique.
*/

struct Solution;

impl Solution {
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        use std::collections::BTreeSet;
        let n = nums.len();
        let mut sum = vec![0i64; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }

        let mut cuts = BTreeSet::<i32>::new();
        let mut s = BTreeSet::<(i64, i32, i32)>::new();
        let mut ret: Vec<i64> = vec![];

        cuts.insert(-1);
        cuts.insert(n as i32);
        s.insert((sum[n], -1, n as i32));

        for r in remove_queries {
            let left = if let Some(a) = cuts.range(..r).rev().next() {
                *a
            } else {
                0
            };
            let right = if let Some(a) = cuts.range(r..).next() {
                *a
            } else {
                n as i32
            };
            let total = sum[right as usize] - sum[(left + 1) as usize];

            s.remove(&(total, left, right));
            cuts.insert(r);

            if left + 1 < r {
                let total = sum[r as usize] - sum[(left + 1) as usize];
                s.insert((total, left, r));
            }

            if r + 1 < right {
                let total = sum[right as usize] - sum[(r + 1) as usize];
                s.insert((total, r, right));
            }

            if let Some((val, _, _)) = s.iter().next_back() {
                ret.push(*val);
            } else {
                ret.push(0);
            }
        }

        ret
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 5, 6, 1];
    let remove_queries = vec![0, 3, 2, 4, 1];
    let ret = Solution::maximum_segment_sum(nums, remove_queries);
    assert_eq!(ret, vec![14, 7, 2, 2, 0]);

    let nums = vec![3, 2, 11, 1];
    let remove_queries = vec![3, 2, 1, 0];
    let ret = Solution::maximum_segment_sum(nums, remove_queries);
    assert_eq!(ret, vec![16, 5, 3, 0]);
}
