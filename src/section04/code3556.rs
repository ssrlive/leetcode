#![allow(dead_code)]

// 3556. Sum of Largest Prime Substrings
// https://leetcode.com/problems/sum-of-largest-prime-substrings/
// https://leetcode.cn/problems/sum-of-largest-prime-substrings/
//
// Medium
//
// Given a string s, find the sum of the 3 largest unique that can be formed using any of its.
//
// Return the sum of the three largest unique prime numbers that can be formed. If fewer than three exist,
// return the sum of all available primes. If no prime numbers can be formed, return 0.
//
// Note: Each prime number should be counted only once, even if it appears in multiple substrings.
// Additionally, when converting a substring to an integer, any leading zeros are ignored.
//
// Example 1:
//
// Input: s = "12234"
//
// Output: 1469
//
// Explanation:
//
//     The unique prime numbers formed from the substrings of "12234" are 2, 3, 23, 223, and 1223.
//     The 3 largest primes are 1223, 223, and 23. Their sum is 1469.
//
// Example 2:
//
// Input: s = "111"
//
// Output: 11
//
// Explanation:
//
//     The unique prime number formed from the substrings of "111" is 11.
//     Since there is only one prime number, the sum is 11.
//
// Constraints:
//
//     1 <= s.length <= 10
//     s consists of only digits.
//

struct Solution;

impl Solution {
    pub fn sum_of_largest_primes(s: String) -> i64 {
        fn is_prime(n: i64) -> bool {
            if n < 2 {
                return false;
            }
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    return false;
                }
                i += 1;
            }
            true
        }

        let mut primes = vec![];
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            let mut temp = 0;
            for &bytes_j in bytes.iter().skip(i) {
                temp = temp * 10 + (bytes_j - b'0') as i64;
                if is_prime(temp) {
                    primes.push(temp);
                }
            }
        }
        primes.sort_unstable();
        primes.dedup();
        let m = primes.len();
        if m < 3 {
            return primes.iter().sum();
        }
        primes[m - 1] + primes[m - 2] + primes[m - 3]
    }
}

#[test]
fn test() {
    let s = "12234".to_string();
    assert_eq!(Solution::sum_of_largest_primes(s), 1469);

    let s = "111".to_string();
    assert_eq!(Solution::sum_of_largest_primes(s), 11);

    let s = "1234567890".to_string();
    assert_eq!(Solution::sum_of_largest_primes(s), 23461445);
}
