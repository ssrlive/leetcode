#![allow(dead_code)]

/*

// 1655. Distribute Repeating Integers
// https://leetcode.com/problems/distribute-repeating-integers/
// https://leetcode.cn/problems/distribute-repeating-integers/
//
// Hard
//
// You are given an array of n integers, nums, where there are at most 50 unique values in the array. You are also given an array of m customer order quantities, quantity, where quantity[i] is the amount of integers the ith customer ordered. Determine if it is possible to distribute nums such that:

    The ith customer gets exactly quantity[i] integers,
    The integers the ith customer gets are all equal, and
    Every customer is satisfied.

Return true if it is possible to distribute nums according to the above conditions.

Example 1:

Input: nums = [1,2,3,4], quantity = [2]
Output: false
Explanation: The 0th customer cannot be given two different integers.

Example 2:

Input: nums = [1,2,3,3], quantity = [2]
Output: true
Explanation: The 0th customer is given [3,3]. The integers [1,2] are not used.

Example 3:

Input: nums = [1,1,2,2], quantity = [2,2]
Output: true
Explanation: The 0th customer is given [1,1], and the 1st customer is given [2,2].

Constraints:

    n == nums.length
    1 <= n <= 10^5
    1 <= nums[i] <= 1000
    m == quantity.length
    1 <= m <= 10
    1 <= quantity[i] <= 10^5
    There are at most 50 unique values in nums.
*/

struct Solution;

impl Solution {
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        use std::collections::HashMap;

        fn backtrack(counts: &mut Vec<i32>, quantity: &Vec<i32>, index: usize) -> bool {
            if index == quantity.len() {
                return true;
            }
            for i in 0..counts.len() {
                if counts[i] >= quantity[index] {
                    let p = quantity[index];
                    counts[i] -= p;
                    if backtrack(counts, quantity, index + 1) {
                        return true;
                    }
                    counts[i] += p;
                }
            }
            false
        }

        let mut counts = HashMap::new();
        for n in nums.iter() {
            *counts.entry(*n).or_insert(0) += 1;
        }
        let mut counts = counts.values().copied().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        let mut quantity = quantity;
        quantity.sort_by(|a, b| b.cmp(a));

        backtrack(&mut counts, &quantity, 0)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], vec![2], false),
        (vec![1, 2, 3, 3], vec![2], true),
        (vec![1, 1, 2, 2], vec![2, 2], true),
        (vec![1, 1, 2, 3], vec![2, 2], false),
        (vec![1, 1, 1, 1, 1], vec![2, 3], true),
        (vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], vec![2, 2, 2, 2, 2], true),
    ];
    for (nums, quantity, expected) in cases {
        assert_eq!(Solution::can_distribute(nums, quantity), expected);
    }
}
