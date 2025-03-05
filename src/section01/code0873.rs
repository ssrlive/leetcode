#![allow(dead_code)]

// 873. Length of Longest Fibonacci Subsequence
// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
// https://leetcode.cn/problems/length-of-longest-fibonacci-subsequence/
//
// A sequence x1, x2, ..., xn is Fibonacci-like if:
//
// n >= 3
// xi + xi+1 == xi+2 for all i + 2 <= n
// Given a strictly increasing array arr of positive integers forming a sequence, return the length
// of the longest Fibonacci-like subsequence of arr. If one does not exist, return 0.
//
// A subsequence is derived from another sequence arr by deleting any number of elements (including none) from arr,
// without changing the order of the remaining elements. For example, [3, 5, 8] is a subsequence of [3, 4, 5, 6, 7, 8].
//
// Example 1:
//
// Input: arr = [1,2,3,4,5,6,7,8]
// Output: 5
// Explanation: The longest subsequence that is fibonacci-like: [1,2,3,5,8].
//
// Example 2:
//
// Input: arr = [1,3,7,11,12,14,18]
// Output: 3
// Explanation: The longest subsequence that is fibonacci-like: [1,11,12], [3,11,14] or [7,11,18].
//
// Constraints:
//
// - 3 <= arr.length <= 1000
// - 1 <= arr[i] < arr[i + 1] <= 10^9
//

struct Solution;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut mp = std::collections::HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            mp.insert(v, i);
        }
        let mut temp = 0;
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                let mut sum = 2;
                let mut first = arr[i];
                let mut second = arr[j];
                while mp.contains_key(&(first + second)) {
                    sum += 1;
                    let third = first + second;
                    first = second;
                    second = third;
                }
                temp = std::cmp::max(temp, sum);
            }
        }
        if temp >= 3 { temp } else { 0 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
    assert_eq!(Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
}
