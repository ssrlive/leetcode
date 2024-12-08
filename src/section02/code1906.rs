#![allow(dead_code)]

/*

// 1906. Minimum Absolute Difference Queries
// https://leetcode.com/problems/minimum-absolute-difference-queries/
// https://leetcode.cn/problems/minimum-absolute-difference-queries/
//
// Medium
//
// The minimum absolute difference of an array a is defined as the minimum value of |a[i] - a[j]|, where 0 <= i < j < a.length and a[i] != a[j]. If all elements of a are the same, the minimum absolute difference is -1.

    For example, the minimum absolute difference of the array [5,2,3,7,2] is |2 - 3| = 1. Note that it is not 0 because a[i] and a[j] must be different.

You are given an integer array nums and the array queries where queries[i] = [li, ri]. For each query i, compute the minimum absolute difference of the subarray nums[li...ri] containing the elements of nums between the 0-based indices li and ri (inclusive).

Return an array ans where ans[i] is the answer to the ith query.

A subarray is a contiguous sequence of elements in an array.

The value of |x| is defined as:

    x if x >= 0.
    -x if x < 0.

Example 1:

Input: nums = [1,3,4,8], queries = [[0,1],[1,2],[2,3],[0,3]]
Output: [2,1,4,1]
Explanation: The queries are processed as follows:
- queries[0] = [0,1]: The subarray is [1,3] and the minimum absolute difference is |1-3| = 2.
- queries[1] = [1,2]: The subarray is [3,4] and the minimum absolute difference is |3-4| = 1.
- queries[2] = [2,3]: The subarray is [4,8] and the minimum absolute difference is |4-8| = 4.
- queries[3] = [0,3]: The subarray is [1,3,4,8] and the minimum absolute difference is |3-4| = 1.

Example 2:

Input: nums = [4,5,2,2,7,10], queries = [[2,3],[0,2],[0,5],[3,5]]
Output: [-1,1,1,3]
Explanation: The queries are processed as follows:
- queries[0] = [2,3]: The subarray is [2,2] and the minimum absolute difference is -1 because all the
  elements are the same.
- queries[1] = [0,2]: The subarray is [4,5,2] and the minimum absolute difference is |4-5| = 1.
- queries[2] = [0,5]: The subarray is [4,5,2,2,7,10] and the minimum absolute difference is |4-5| = 1.
- queries[3] = [3,5]: The subarray is [2,7,10] and the minimum absolute difference is |7-10| = 3.

Constraints:

    2 <= nums.length <= 10^5
    1 <= nums[i] <= 100
    1 <= queries.length <= 2 * 10^4
    0 <= li < ri < nums.length
*/

struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let tree = SegmentTree::new(0, nums.len() - 1, &nums);
        queries
            .into_iter()
            .map(|r| {
                let start = r[0] as usize;
                let end = r[1] as usize;
                let bitmask = tree.query(start, end);
                let mut prev: Option<usize> = None;
                let mut res: Option<usize> = None;
                for i in 0..100 {
                    if (bitmask >> i) & 1 == 0 {
                        continue;
                    }
                    match (prev, res) {
                        (None, None) => {
                            prev = Some(i);
                        }
                        (Some(prev_), None) => {
                            prev = Some(i);
                            res = Some(i - prev_);
                        }
                        (Some(prev_), Some(res_)) => {
                            prev = Some(i);
                            res = Some(res_.min(i - prev_));
                        }
                        _ => (),
                    }
                }
                res.map(|a| a as i32).unwrap_or(-1)
            })
            .collect()
    }
}

pub struct SegmentTree {
    pub start: usize,
    pub end: usize,
    pub bitmask: u128,
    pub left: Option<Box<SegmentTree>>,
    pub right: Option<Box<SegmentTree>>,
}

impl SegmentTree {
    pub fn new(start: usize, end: usize, vals: &[i32]) -> Self {
        if start == end {
            let bitmask = 1 << (vals[start] - 1);
            return Self {
                start,
                end,
                bitmask,
                left: None,
                right: None,
            };
        }
        let mid = start + (end - start) / 2;
        let left = Self::new(start, mid, vals);
        let right = Self::new(mid + 1, end, vals);
        Self {
            start,
            end,
            bitmask: left.bitmask | right.bitmask,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn query(&self, start: usize, end: usize) -> u128 {
        if self.start == start && self.end == end {
            return self.bitmask;
        }
        let mid = self.start + (self.end - self.start) / 2;
        if end <= mid {
            self.left.as_ref().unwrap().query(start, end)
        } else if start > mid {
            self.right.as_ref().unwrap().query(start, end)
        } else {
            let left = self.left.as_ref().unwrap().query(start, mid);
            let right = self.right.as_ref().unwrap().query(mid + 1, end);
            left | right
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 4, 8];
    let queries = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]];
    let res = vec![2, 1, 4, 1];
    assert_eq!(Solution::min_difference(nums, queries), res);

    let nums = vec![4, 5, 2, 2, 7, 10];
    let queries = vec![vec![2, 3], vec![0, 2], vec![0, 5], vec![3, 5]];
    let res = vec![-1, 1, 1, 3];
    assert_eq!(Solution::min_difference(nums, queries), res);
}
