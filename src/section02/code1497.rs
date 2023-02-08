#![allow(dead_code)]

/*

// 1497. Check If Array Pairs Are Divisible by k
// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/
// https://leetcode.cn/problems/check-if-array-pairs-are-divisible-by-k/
//
// Medium
//
// Given an array of integers arr of even length n and an integer k.

We want to divide the array into exactly n / 2 pairs such that the sum of each pair is divisible by k.

Return true If you can find a way to do that or false otherwise.

Example 1:

Input: arr = [1,2,3,4,5,10,6,7,8,9], k = 5
Output: true
Explanation: Pairs are (1,9),(2,8),(3,7),(4,6) and (5,10).

Example 2:

Input: arr = [1,2,3,4,5,6], k = 7
Output: true
Explanation: Pairs are (1,6),(2,5) and(3,4).

Example 3:

Input: arr = [1,2,3,4,5,6], k = 10
Output: false
Explanation: You can try all possible pairs to see that there is no way to divide arr into 3 pairs each with sum divisible by 10.

Constraints:

    arr.length == n
    1 <= n <= 10^5
    n is even.
    -10^9 <= arr[i] <= 10^9
    1 <= k <= 10^5
*/

struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];
        for i in arr {
            count[((i % k) + k) as usize % k as usize] += 1;
        }
        for i in 1..k as usize / 2 + 1 {
            if count[i] != count[k as usize - i] {
                return false;
            }
        }
        count[0] % 2 == 0
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5, true),
        (vec![1, 2, 3, 4, 5, 6], 7, true),
        (vec![1, 2, 3, 4, 5, 6], 10, false),
    ];
    for (arr, k, expected) in cases {
        assert_eq!(Solution::can_arrange(arr, k), expected);
    }
}
