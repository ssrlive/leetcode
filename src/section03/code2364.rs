#![allow(dead_code)]

/*

// 2364. Count Number of Bad Pairs
// https://leetcode.com/problems/count-number-of-bad-pairs/
// https://leetcode.cn/problems/count-number-of-bad-pairs/
//
// Medium
//
// You are given a 0-indexed integer array nums. A pair of indices (i, j) is a bad pair if i < j and j - i != nums[j] - nums[i].

Return the total number of bad pairs in nums.

Example 1:

Input: nums = [4,1,3,3]
Output: 5
Explanation: The pair (0, 1) is a bad pair since 1 - 0 != 1 - 4.
The pair (0, 2) is a bad pair since 2 - 0 != 3 - 4, 2 != -1.
The pair (0, 3) is a bad pair since 3 - 0 != 3 - 4, 3 != -1.
The pair (1, 2) is a bad pair since 2 - 1 != 3 - 1, 1 != 2.
The pair (2, 3) is a bad pair since 3 - 2 != 3 - 3, 1 != 0.
There are a total of 5 bad pairs, so we return 5.

Example 2:

Input: nums = [1,2,3,4,5]
Output: 0
Explanation: There are no bad pairs.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut res = 0;
        nums.iter().zip(0..).for_each(|(&x, i)| {
            let delta = x - i;
            let e = hm.entry(delta).or_default();
            res += (i - *e) as i64;
            *e += 1;
        });
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_bad_pairs(vec![4, 1, 3, 3]), 5);
    assert_eq!(Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
}
