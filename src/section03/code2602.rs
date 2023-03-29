#![allow(dead_code)]

/*
// 2602. Minimum Operations to Make All Array Elements Equal
// https://leetcode.com/problems/minimum-operations-to-make-all-array-elements-equal/
// https://leetcode.cn/problems/minimum-operations-to-make-all-array-elements-equal/
//
// Medium
//
// You are given an array nums consisting of positive integers.

You are also given an integer array queries of size m. For the ith query, you want to make all of the elements of nums equal to queries[i]. You can perform the following operation on the array any number of times:

Increase or decrease an element of the array by 1.
Return an array answer of size m where answer[i] is the minimum number of operations to make all elements of nums equal to queries[i].

Note that after each query the array is reset to its original state.

Example 1:

Input: nums = [3,1,6,8], queries = [1,5]
Output: [14,10]
Explanation: For the first query we can do the following operations:
- Decrease nums[0] 2 times, so that nums = [1,1,6,8].
- Decrease nums[2] 5 times, so that nums = [1,1,1,8].
- Decrease nums[3] 7 times, so that nums = [1,1,1,1].
So the total number of operations for the first query is 2 + 5 + 7 = 14.
For the second query we can do the following operations:
- Increase nums[0] 2 times, so that nums = [5,1,6,8].
- Increase nums[1] 4 times, so that nums = [5,5,6,8].
- Decrease nums[2] 1 time, so that nums = [5,5,5,8].
- Decrease nums[3] 3 times, so that nums = [5,5,5,5].
So the total number of operations for the second query is 2 + 4 + 1 + 3 = 10.
Example 2:

Input: nums = [2,9,6,3], queries = [10]
Output: [20]
Explanation: We can increase each value in the array to 10. The total number of operations will be 8 + 1 + 4 + 7 = 20.

Constraints:

n == nums.length
m == queries.length
1 <= n, m <= 10^5
1 <= nums[i], queries[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        fn upper_bound<T: Ord>(arr: &[T], x: &T) -> Result<usize, usize> {
            let mut left = 0;
            let len = arr.len();
            let mut right = len;
            while left < right {
                let mid = left + (right - left) / 2;
                match arr[mid].cmp(x) {
                    std::cmp::Ordering::Greater => right = mid,
                    _ => left = mid + 1,
                }
            }
            assert_eq!(left, right);
            if left == len {
                Err(left)
            } else {
                Ok(left)
            }
        }

        let mut ps = vec![0];
        let mut nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        nums.sort();
        for n in nums.iter() {
            ps.push(ps.last().unwrap() + n);
        }
        let queries = queries.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut res = vec![];
        for q in queries.iter() {
            let i = upper_bound(&nums, q).unwrap_or_else(|e| e);
            res.push(q * i as i64 - ps[i] + (ps.last().unwrap() - ps[i]) - q * (nums.len() - i) as i64);
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 1, 6, 8], vec![1, 5], vec![14, 10]),
        (vec![2, 9, 6, 3], vec![10], vec![20]),
    ];
    for (nums, queries, expect) in cases {
        assert_eq!(Solution::min_operations(nums, queries), expect);
    }
}
