#![allow(dead_code)]

/*

// 2607. Make K-Subarray Sums Equal
// https://leetcode.com/problems/make-k-subarray-sums-equal/
// https://leetcode.cn/problems/make-k-subarray-sums-equal/
//
// Medium
//
// You are given a 0-indexed integer array arr and an integer k. The array arr is circular.
// In other words, the first element of the array is the next element of the last element,
// and the last element of the array is the previous element of the first element.

You can do the following operation any number of times:

    Pick any element from arr and increase or decrease it by 1.

Return the minimum number of operations such that the sum of each subarray of length k is equal.

A subarray is a contiguous part of the array.

Example 1:

Input: arr = [1,4,1,3], k = 2
Output: 1
Explanation: we can do one operation on index 1 to make its value equal to 3.
The array after the operation is [1,3,1,3]
- Subarray starts at index 0 is [1, 3], and its sum is 4
- Subarray starts at index 1 is [3, 1], and its sum is 4
- Subarray starts at index 2 is [1, 3], and its sum is 4
- Subarray starts at index 3 is [3, 1], and its sum is 4

Example 2:

Input: arr = [2,5,5,7], k = 3
Output: 5
Explanation: we can do three operations on index 0 to make its value equal to 5 and two operations on index 3 to make its value equal to 5.
The array after the operations is [5,5,5,5]
- Subarray starts at index 0 is [5, 5, 5], and its sum is 15
- Subarray starts at index 1 is [5, 5, 5], and its sum is 15
- Subarray starts at index 2 is [5, 5, 5], and its sum is 15
- Subarray starts at index 3 is [5, 5, 5], and its sum is 15

Constraints:

    1 <= k <= arr.length <= 10^5
    1 <= arr[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let arr = arr.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let k = k as i64;
        let n = arr.len() as i64;
        let k = gcd(n, k);
        let mut ans = 0;
        for i in 0..k {
            let mut vec = vec![];
            for j in (i..n).step_by(k as usize) {
                vec.push(arr[j as usize]);
            }
            vec.sort();
            let mid = vec.len() / 2;
            for ind in 0..vec.len() {
                ans += (vec[mid] - vec[ind]).abs();
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 4, 1, 3], 2, 1),
        (vec![2, 5, 5, 7], 3, 5),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 25),
    ];
    for (arr, k, expected) in cases {
        assert_eq!(Solution::make_sub_k_sum_equal(arr, k), expected);
    }
}
