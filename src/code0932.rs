#![allow(dead_code)]

// 932. Beautiful Array
// https://leetcode.com/problems/beautiful-array/
// https://leetcode.cn/problems/beautiful-array/
//
// An array nums of length n is beautiful if:
//
// nums is a permutation of the integers in the range [1, n].
// For every 0 <= i < j < n, there is no index k with i < k < j where 2 * nums[k] == nums[i] + nums[j].
// Given the integer n, return any beautiful array nums of length n. There will be at least one valid answer for the given n.
//
// Example 1:
//
// Input: n = 4
// Output: [2,1,4,3]
//
// Example 2:
//
// Input: n = 5
// Output: [3,1,2,5,4]
//
// Constraints:
//
// - 1 <= n <= 1000
//

struct Solution;

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(n as usize);
        answer.push(1);
        while answer.len() < n as usize {
            answer = answer
                .iter()
                .map(|m| m * 2 - 1)
                .chain(answer.iter().map(|m| m * 2))
                .filter(|&m| m <= n)
                .collect();
        }
        answer
    }
}

#[test]
fn test() {
    let mut result = Solution::beautiful_array(4);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4]);

    let mut result = Solution::beautiful_array(5);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}
