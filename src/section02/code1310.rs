#![allow(dead_code)]

// 1310. XOR Queries of a Subarray
// https://leetcode.com/problems/xor-queries-of-a-subarray/
// https://leetcode.cn/problems/xor-queries-of-a-subarray/
//
// Medium
//
// You are given an array arr of positive integers. You are also given the array queries where queries[i] = [lefti, righti].
//
// For each query i compute the XOR of elements from lefti to righti (that is, arr[lefti] XOR arr[lefti + 1] XOR ... XOR arr[righti] ).
//
// Return an array answer where answer[i] is the answer to the ith query.
//
// Example 1:
//
// Input: arr = [1,3,4,8], queries = [[0,1],[1,2],[0,3],[3,3]]
// Output: [2,7,14,8]
// Explanation:
// The binary representation of the elements in the array are:
// 1 = 0001
// 3 = 0011
// 4 = 0100
// 8 = 1000
// The XOR values for queries are:
// [0,1] = 1 xor 3 = 2
// [1,2] = 3 xor 4 = 7
// [0,3] = 1 xor 3 xor 4 xor 8 = 14
// [3,3] = 8
//
// Example 2:
//
// Input: arr = [4,8,2,10], queries = [[2,3],[1,3],[0,0],[0,3]]
// Output: [8,0,4,4]
//
// Constraints:
//
// -    1 <= arr.length, queries.length <= 3 * 10^4
// -    1 <= arr[i] <= 10^9
// -    queries[i].length == 2
// -    0 <= lefti <= righti < arr.length
//

struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = arr;
        for i in 1..arr.len() {
            arr[i] ^= arr[i - 1];
        }

        queries
            .iter()
            .map(|q| arr[q[1] as usize] ^ if q[0] == 0 { 0 } else { arr[q[0] as usize - 1] })
            .collect()
    }
}

#[test]
fn test() {
    let arr = vec![1, 3, 4, 8];
    let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
    let result = Solution::xor_queries(arr, queries);
    assert_eq!(result, vec![2, 7, 14, 8]);

    let arr = vec![4, 8, 2, 10];
    let queries = vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]];
    let result = Solution::xor_queries(arr, queries);
    assert_eq!(result, vec![8, 0, 4, 4]);
}
