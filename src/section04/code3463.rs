#![allow(dead_code)]

// 3463. Check If Digits Are Equal in String After Operations II
// https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-ii/
// https://leetcode.cn/problems/check-if-digits-are-equal-in-string-after-operations-ii/
//
// Hard
//
// You are given a string s consisting of digits. Perform the following operation repeatedly until the string has exactly two digits:
//
// For each pair of consecutive digits in s, starting from the first digit, calculate a new digit as the sum of the two digits modulo 10.
// Replace s with the sequence of newly calculated digits, maintaining the order in which they are computed.
// Return true if the final two digits in s are the same; otherwise, return false.
//
// Example 1:
//
// Input: s = "3902"
//
// Output: true
//
// Explanation:
//
// Initially, s = "3902"
// First operation:
// (s[0] + s[1]) % 10 = (3 + 9) % 10 = 2
// (s[1] + s[2]) % 10 = (9 + 0) % 10 = 9
// (s[2] + s[3]) % 10 = (0 + 2) % 10 = 2
// s becomes "292"
// Second operation:
// (s[0] + s[1]) % 10 = (2 + 9) % 10 = 1
// (s[1] + s[2]) % 10 = (9 + 2) % 10 = 1
// s becomes "11"
// Since the digits in "11" are the same, the output is true.
//
// Example 2:
//
// Input: s = "34789"
//
// Output: false
//
// Explanation:
//
// Initially, s = "34789".
// After the first operation, s = "7157".
// After the second operation, s = "862".
// After the third operation, s = "48".
// Since '4' != '8', the output is false.
//
// Constraints:
//
// 3 <= s.length <= 10^5
// s consists of only digits.
//

struct Solution;

/*
class Solution {
public:
    int binmod10(int n, int k) {
        int mod2 = binmod2(n, k);
        int mod5 = binmod5(n, k);
        for (int i = 0; i < 10; i++) {
            if (i % 2 == mod2 && i % 5 == mod5)
                return i;
        }
        return 0;
    }

    int binmod2(int n, int k) {
        while (k > 0) {
            if ((k & 1) > (n & 1))
                return 0;
            n >>= 1;
            k >>= 1;
        }
        return 1;
    }

    int binmod5(int n, int k) {
        int res = 1;
        while (n > 0 || k > 0) {
            int nd = n % 5;
            int kd = k % 5;
            if (kd > nd)
                return 0;
            res = (res * binsmall(nd, kd)) % 5;
            n /= 5;
            k /= 5;
        }
        return res;
    }

    int binsmall(int n, int k) {
        if (k > n)
            return 0;
        int fact[5] = {1, 1, 2, 1, 4};
        int numerator = fact[n];
        int denominator = (fact[k] * fact[n - k]) % 5;
        int deninv = 0;
        for (int i = 0; i < 5; i++) {
            if ((denominator * i) % 5 == 1) {
                deninv = i;
                break;
            }
        }
        return (numerator * deninv) % 5;
    }

    bool hasSameDigits(string s) {
        int L = s.size();
        int m = L - 2;
        int s1 = 0, s2 = 0;
        for (int i = 0; i <= m; i++) {
            int val = binmod10(m, i);
            int d1 = s[i] - '0';
            int d2 = s[i + 1] - '0';
            s1 = (s1 + val * d1) % 10;
            s2 = (s2 + val * d2) % 10;
        }
        return s1 == s2;
    }
};
*/

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        fn binmod10(n: i32, k: i32) -> i32 {
            let mod2 = binmod2(n, k);
            let mod5 = binmod5(n, k);
            for i in 0..10 {
                if i % 2 == mod2 && i % 5 == mod5 {
                    return i;
                }
            }
            0
        }

        fn binmod2(mut n: i32, mut k: i32) -> i32 {
            while k > 0 {
                if (k & 1) > (n & 1) {
                    return 0;
                }
                n >>= 1;
                k >>= 1;
            }
            1
        }

        fn binmod5(mut n: i32, mut k: i32) -> i32 {
            let mut res = 1;
            while n > 0 || k > 0 {
                let nd = n % 5;
                let kd = k % 5;
                if kd > nd {
                    return 0;
                }
                res = (res * binsmall(nd, kd)) % 5;
                n /= 5;
                k /= 5;
            }
            res
        }

        fn binsmall(n: i32, k: i32) -> i32 {
            if k > n {
                return 0;
            }
            let fact = [1, 1, 2, 1, 4];
            let numerator = fact[n as usize];
            let denominator = (fact[k as usize] * fact[(n - k) as usize]) % 5;
            let mut deninv = 0;
            for i in 0..5 {
                if (denominator * i) % 5 == 1 {
                    deninv = i;
                    break;
                }
            }
            (numerator * deninv) % 5
        }

        let s = s.as_bytes();
        let l = s.len();
        let m = l - 2;
        let mut s1 = 0;
        let mut s2 = 0;
        for i in 0..=m {
            let val = binmod10(m as i32, i as i32);
            let d1 = (s[i] - b'0') as i32;
            let d2 = (s[i + 1] - b'0') as i32;
            s1 = (s1 + val * d1) % 10;
            s2 = (s2 + val * d2) % 10;
        }
        s1 == s2
    }
}

#[test]
fn test() {
    assert!(Solution::has_same_digits("3902".to_string()));
    assert!(!Solution::has_same_digits("34789".to_string()));
}
