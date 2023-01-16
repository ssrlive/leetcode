#![allow(dead_code)]

// 667. Beautiful Arrangement II
// https://leetcode.com/problems/beautiful-arrangement-ii/
// https://leetcode.cn/problems/beautiful-arrangement-ii/
//
// Given two integers n and k, construct a list answer that contains n different positive integers ranging
// from 1 to n and obeys the following requirement:
//
// Suppose this list is answer = [a1, a2, a3, ... , an], then the list [|a1 - a2|, |a2 - a3|, |a3 - a4|, ... , |an-1 - an|]
// has exactly k distinct integers.
// Return the list answer. If there multiple valid answers, return any of them.
//
// Example 1:
//
// Input: n = 3, k = 1
// Output: [1,2,3]
// Explanation: The [1,2,3] has three different positive integers ranging from 1 to 3, and the [1,1] has exactly 1 distinct integer: 1
//
// Example 2:
//
// Input: n = 3, k = 2
// Output: [1,3,2]
// Explanation: The [1,3,2] has three different positive integers ranging from 1 to 3, and the [2,1] has exactly 2 distinct integers: 1 and 2.
//
// Constraints:
//
// - 1 <= k < n <= 10^4
//

struct Solution;

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut k = k;
        let mut result = Vec::with_capacity(n as usize);
        let mut i = 1;
        let mut j = n;
        while i <= j {
            if k % 2 == 1 {
                result.push(i);
                i += 1;
            } else {
                result.push(j);
                j -= 1;
            }
            if k > 1 {
                k -= 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    let mut res = Solution::construct_array(3, 1);
    res.sort();
    assert_eq!(res, vec![1, 2, 3]);

    let mut res = Solution::construct_array(3, 2);
    res.sort();
    assert_eq!(res, vec![1, 2, 3]);
}
