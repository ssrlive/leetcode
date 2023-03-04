#![allow(dead_code)]

/*

// 2164. Sort Even and Odd Indices Independently
// https://leetcode.com/problems/sort-even-and-odd-indices-independently/
// https://leetcode.cn/problems/sort-even-and-odd-indices-independently/
//
// Easy
//
// You are given a 0-indexed integer array nums. Rearrange the values of nums according to the following rules:

    Sort the values at odd indices of nums in non-increasing order.
        For example, if nums = [4,1,2,3] before this step, it becomes [4,3,2,1] after. The values at odd indices 1 and 3 are sorted in non-increasing order.
    Sort the values at even indices of nums in non-decreasing order.
        For example, if nums = [4,1,2,3] before this step, it becomes [2,1,4,3] after. The values at even indices 0 and 2 are sorted in non-decreasing order.

Return the array formed after rearranging the values of nums.

Example 1:

Input: nums = [4,1,2,3]
Output: [2,3,4,1]
Explanation:
First, we sort the values present at odd indices (1 and 3) in non-increasing order.
So, nums changes from [4,1,2,3] to [4,3,2,1].
Next, we sort the values present at even indices (0 and 2) in non-decreasing order.
So, nums changes from [4,1,2,3] to [2,3,4,1].
Thus, the array formed after rearranging the values is [2,3,4,1].

Example 2:

Input: nums = [2,1]
Output: [2,1]
Explanation:
Since there is exactly one odd index and one even index, no rearrangement of values takes place.
The resultant array formed is [2,1], which is the same as the initial array.

Constraints:

    1 <= nums.length <= 100
    1 <= nums[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 3 {
            return nums;
        }
        let mut evens = nums.iter().step_by(2).collect::<Vec<_>>();
        let mut odds = nums[1..].iter().step_by(2).collect::<Vec<_>>();
        evens.sort_unstable();
        odds.sort_unstable_by_key(|&&x| -x);
        let (mut evens_it, mut odds_it) = (evens.into_iter(), odds.into_iter());
        let mut result = Vec::new();
        let mut any = 1;
        while any != 0 {
            any = 0;
            if let Some(&even) = evens_it.next() {
                result.push(even);
                any |= 1;
            }
            if let Some(&odd) = odds_it.next() {
                result.push(odd);
                any |= 2;
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 1, 2, 3], vec![2, 3, 4, 1]),
        (vec![2, 1], vec![2, 1]),
        (vec![2, 1, 3], vec![2, 1, 3]),
        (vec![2, 1, 3, 4], vec![2, 4, 3, 1]),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::sort_even_odd(nums), expected);
    }
}
