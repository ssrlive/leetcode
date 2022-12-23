#![allow(dead_code)]

// 60. Permutation Sequence
// https://leetcode.com/problems/permutation-sequence/
// https://leetcode.com/problems/permutation-sequence/
//
// The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
//
// By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
//
// "123"
// "132"
// "213"
// "231"
// "312"
// "321"
//
// Given n and k, return the kth permutation sequence.
//
// Example 1:
//
// Input: n = 3, k = 3
// Output: "213"
//
// Example 2:
//
// Input: n = 4, k = 9
// Output: "2314"
//
// Example 3:
//
// Input: n = 3, k = 1
// Output: "123"
//
// Constraints:
//
// - 1 <= n <= 9
// - 1 <= k <= n!
//

struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut perm = vec![1];
        let mut tot = 1;
        for i in 1..n {
            tot *= i;
            perm.push(tot);
        }
        let mut k = k - 1;
        let mut ans = String::new();
        let mut arr: Vec<i32> = (1..=n).collect();

        for _ in 0..n {
            let fac = perm.pop().unwrap_or(0);
            let mut ai: usize = 0;
            if k >= fac {
                ai = (k / fac) as usize;
                k -= ai as i32 * fac;
            }
            let d = arr.remove(ai);

            let c = char::from_digit(d as u32, 10).unwrap_or('0');
            ans.push(c);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(3, 3, "213"), (4, 9, "2314"), (3, 1, "123")];
    for (n, k, expected) in cases {
        assert_eq!(Solution::get_permutation(n, k), expected);
    }
}
