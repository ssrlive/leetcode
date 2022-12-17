#![allow(dead_code)]

// 823. Binary Trees With Factors
// https://leetcode.com/problems/binary-trees-with-factors/
//
// Given an array of unique integers, arr, where each integer arr[i] is strictly greater than 1.
//
// We make a binary tree using these integers, and each number may be used for any number of times.
// Each non-leaf node's value should be equal to the product of the values of its children.
//
// Return the number of binary trees we can make. The answer may be too large so return the answer modulo 109 + 7.
//
// Example 1:
//
// Input: arr = [2,4]
// Output: 3
// Explanation: We can make these trees: [2], [4], [4, 2, 2]
//
// Example 2:
//
// Input: arr = [2,4,5,10]
// Output: 7
//
// Explanation: We can make these trees: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].
//
// Constraints:
//
// - 1 <= arr.length <= 1000
// - 2 <= arr[i] <= 109
// - All the values of arr are unique.
//

struct Solution;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        fn _num_factored_binary_trees(arr: Vec<i64>) -> i64 {
            let mut arr = arr;
            arr.sort_unstable();
            let mut map = std::collections::HashMap::new();
            for (i, &item) in arr.iter().enumerate() {
                map.insert(item, i);
            }
            let mut dp = vec![1; arr.len()];
            for i in 0..arr.len() {
                for j in 0..i {
                    if arr[i] % arr[j] == 0 {
                        if let Some(&k) = map.get(&(arr[i] / arr[j])) {
                            dp[i] += dp[j] * dp[k];
                            dp[i] %= 1_000_000_007;
                        }
                    }
                }
            }
            dp.iter().sum::<i64>() % 1_000_000_007
        }

        _num_factored_binary_trees(arr.into_iter().map(|x| x as i64).collect()) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
    assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
}
