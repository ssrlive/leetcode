#![allow(dead_code)]

/*

// 1569. Number of Ways to Reorder Array to Get Same BST
// https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/
// https://leetcode.cn/problems/number-of-ways-to-reorder-array-to-get-same-bst/
//
// Hard
//
// Given an array nums that represents a permutation of integers from 1 to n. We are going to construct a binary search tree (BST) by inserting the elements of nums in order into an initially empty BST. Find the number of different ways to reorder nums so that the constructed BST is identical to that formed from the original array nums.

    For example, given nums = [2,1,3], we will have 2 as the root, 1 as a left child, and 3 as a right child. The array [2,3,1] also yields the same BST but [3,2,1] yields a different BST.

Return the number of ways to reorder nums such that the BST formed is identical to the original BST formed from nums.

Since the answer may be very large, return it modulo 109 + 7.

Example 1:

Input: nums = [2,1,3]
Output: 1
Explanation: We can reorder nums to be [2,3,1] which will yield the same BST. There are no other ways to reorder nums which will yield the same BST.

Example 2:

Input: nums = [3,4,5,1,2]
Output: 5
Explanation: The following 5 arrays will yield the same BST:
[3,1,2,4,5]
[3,1,4,2,5]
[3,1,4,5,2]
[3,4,1,2,5]
[3,4,1,5,2]

Example 3:

Input: nums = [1,2,3]
Output: 0
Explanation: There are no other orderings of nums that will yield the same BST.

Constraints:

    1 <= nums.length <= 1000
    1 <= nums[i] <= nums.length
    All integers in nums are distinct.
*/

struct Solution;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mod_num = 1_000_000_007;
        let n = nums.len();
        let mut table = vec![vec![1; n + 1]; n + 1];
        for i in 1..n + 1 {
            for j in 1..i {
                table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % mod_num;
            }
        }
        let ans = Self::dfs(&nums, mod_num, &table);
        (ans % mod_num - 1) as i32
    }

    fn dfs(nums: &[i64], mod_num: i64, table: &Vec<Vec<i64>>) -> i64 {
        let n = nums.len();
        if n <= 2 {
            return 1;
        }

        let (mut left, mut right) = (vec![], vec![]);
        for i in 1..nums.len() {
            if nums[i] < nums[0] {
                left.push(nums[i]);
            } else {
                right.push(nums[i]);
            }
        }

        let left_res = Self::dfs(&left, mod_num, table) % mod_num;
        let right_res = Self::dfs(&right, mod_num, table) % mod_num;

        let left_len = left.len();
        table[n - 1][left_len] * left_res % mod_num * right_res % mod_num
    }
}

#[test]
fn test() {
    let cases = vec![(vec![2, 1, 3], 1), (vec![3, 4, 5, 1, 2], 5), (vec![1, 2, 3], 0)];
    for (nums, expected) in cases {
        assert_eq!(Solution::num_of_ways(nums), expected);
    }
}
