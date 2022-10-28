#![allow(dead_code)]

// 216. Combination Sum III
// https://leetcode.com/problems/combination-sum-iii/
//
// Find all valid combinations of k numbers that sum up to n such that the following conditions are true:
//
// - Only numbers 1 through 9 are used.
// - Each number is used at most once.
//
// Return a list of all possible valid combinations. The list must not contain the same combination twice,
// and the combinations may be returned in any order.
//
// Example 1:
// Input: k = 3, n = 7
// Output: [[1,2,4]]
//
// Example 2:
// Input: k = 3, n = 9
// Output: [[1,2,6], [1,3,5], [2,3,4]]
//
// Example 3:
// Input: k = 4, n = 1
// Output: []
//
// Example 4:
// Input: k = 3, n = 2
// Output: []
//
// Example 5:
// Input: k = 9, n = 45
// Output: [[1,2,3,4,5,6,7,8,9]]
//
// Constraints:
// 2 <= k <= 9
// 1 <= n <= 60

struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtrack(result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, k: i32, n: i32, start: i32) {
            if k == 0 && n == 0 {
                result.push(path.clone());
                return;
            }
            if k < 0 || n < 0 {
                return;
            }
            for i in start..=9 {
                path.push(i);
                backtrack(result, path, k - 1, n - i, i + 1);
                path.pop();
            }
        }

        let mut result = Vec::new();
        let mut path = Vec::new();
        backtrack(&mut result, &mut path, k, n, 1);
        result
    }
}

#[test]
fn test_combination_sum3() {
    assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    assert_eq!(
        Solution::combination_sum3(3, 9),
        vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
    );
    assert_eq!(Solution::combination_sum3(4, 1), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::combination_sum3(3, 2), Vec::<Vec<i32>>::new());
    assert_eq!(
        Solution::combination_sum3(9, 45),
        vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]]
    );
}
