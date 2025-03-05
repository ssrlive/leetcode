#![allow(dead_code)]

/*

// 1830. Minimum Number of Operations to Make String Sorted
// https://leetcode.com/problems/minimum-number-of-operations-to-make-string-sorted/
// https://leetcode.cn/problems/minimum-number-of-operations-to-make-string-sorted/
//
// Hard
//
// You are given a string s (0-indexed)​​​​​​. You are asked to perform the following operation on s​​​​​​ until you get a sorted string:

    Find the largest index i such that 1 <= i < s.length and s[i] < s[i - 1].
    Find the largest index j such that i <= j < s.length and s[k] < s[i - 1] for all the possible values of k in the range [i, j] inclusive.
    Swap the two characters at indices i - 1​​​​ and j​​​​​.
    Reverse the suffix starting at index i​​​​​​.

Return the number of operations needed to make the string sorted. Since the answer can be too large, return it modulo 109 + 7.

Example 1:

Input: s = "cba"
Output: 5
Explanation: The simulation goes as follows:
Operation 1: i=2, j=2. Swap s[1] and s[2] to get s="cab", then reverse the suffix starting at 2. Now, s="cab".
Operation 2: i=1, j=2. Swap s[0] and s[2] to get s="bac", then reverse the suffix starting at 1. Now, s="bca".
Operation 3: i=2, j=2. Swap s[1] and s[2] to get s="bac", then reverse the suffix starting at 2. Now, s="bac".
Operation 4: i=1, j=1. Swap s[0] and s[1] to get s="abc", then reverse the suffix starting at 1. Now, s="acb".
Operation 5: i=2, j=2. Swap s[1] and s[2] to get s="abc", then reverse the suffix starting at 2. Now, s="abc".

Example 2:

Input: s = "aabaa"
Output: 2
Explanation: The simulation goes as follows:
Operation 1: i=3, j=4. Swap s[2] and s[4] to get s="aaaab", then reverse the substring starting at 3. Now, s="aaaba".
Operation 2: i=4, j=4. Swap s[3] and s[4] to get s="aaaab", then reverse the substring starting at 4. Now, s="aaaab".

Constraints:

    1 <= s.length <= 3000
    s​​​​​​ consists only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn make_string_sorted(s: String) -> i32 {
        fn mod_pow(x: i64, y: i64, m: i64) -> i64 {
            if y == 0 {
                return 1;
            }
            let p = mod_pow(x, y / 2, m) % m;
            let p = (p * p) % m;
            if y % 2 == 1 { (p * x) % m } else { p }
        }
        let s = s.as_bytes();
        let mut ft = vec![1_i64];
        let mut im = vec![1];
        let mod_num = 1_000_000_007;
        let sz = s.len() as i64;
        for i in 1..=3000 {
            ft.push(i as i64 * ft[i - 1] % mod_num);
            im.push(mod_pow(ft[i], mod_num - 2, mod_num));
        }
        let mut cnt = vec![0; 26];
        let mut res = 1;
        for i in (0..sz).rev() {
            cnt[(s[i as usize] - b'a') as usize] += 1;
            let mut prems = 0;
            for &cnt_j in cnt.iter().take((s[i as usize] - b'a') as usize) {
                prems += cnt_j;
            }
            prems *= ft[(sz - i - 1) as usize];
            prems %= mod_num;
            for n in &cnt {
                prems *= im[*n as usize];
                prems %= mod_num;
            }
            res += prems;
            res %= mod_num;
        }
        (res - 1) as _
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("cba".to_string(), 5),
        ("aabaa".to_string(), 2),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::make_string_sorted(s), expected);
    }
}
