#![allow(dead_code)]

// 2514. Count Anagrams
// https://leetcode.com/problems/count-anagrams/
// https://leetcode.cn/problems/count-anagrams/
//
// You are given a string s containing one or more words. Every consecutive pair of words is separated by a single space ' '.
//
// A string t is an anagram of string s if the ith word of t is a permutation of the ith word of s.
//
// For example, "acb dfe" is an anagram of "abc def", but "def cab" and "adc bef" are not.
// Return the number of distinct anagrams of s. Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "too hot"
// Output: 18
// Explanation: Some of the anagrams of the given string are "too hot", "oot hot", "oto toh", "too toh", and "too oht".
//
// Example 2:
//
// Input: s = "aa"
// Output: 1
// Explanation: There is only one anagram possible for the given string.
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of lowercase English letters and spaces ' '.
// - There is single space between consecutive words.
//

struct Solution;

impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        fn calculate(data: &Vec<i32>) -> i32 {
            let n = data.iter().sum::<i32>();
            let mut ret = 1;
            for k in 2..=n {
                ret = (ret * k as i64) % 1_000_000_007;
            }
            for d in data {
                for k in 2..=*d {
                    ret = (ret * divide(k) as i64) % 1_000_000_007;
                }
            }
            ret as _
        }

        fn divide(a: i32) -> i32 {
            let (mut base, mut ret) = (a as i64, 1);
            let mut m = 1_000_000_005;
            while m > 0 {
                if m % 2 == 1 {
                    ret = (ret * base) % 1_000_000_007;
                }
                m >>= 1;
                base = (base * base) % 1_000_000_007;
            }
            ret as _
        }

        use std::collections::HashMap;
        let mut mp = HashMap::<char, i32>::new();
        let n = s.len();
        let mut s = s.chars().collect::<Vec<char>>();
        s.push(' ');
        let mut ret = 1;

        for (i, &item) in s.iter().enumerate() {
            if i == n || item == ' ' {
                let data = mp.values().copied().collect::<Vec<i32>>();
                ret = (ret * calculate(&data) as i64) % 1_000_000_007;
                mp.clear();
            } else {
                *mp.entry(item).or_insert(0) += 1;
            }
        }

        ret as _
    }
}

#[test]
fn test() {
    let s = "eoblsuqjnpsrfawprqcqxykbududpvimwtvfyvdsgpcn wmyikoakqwjsutgrucubmpatibfzjoewubqgfinxcznzemjckfacxikbfjygaamsidynhjrwjftneeujuymvznxdu fqaeeqcrlvjj hrqhhqrjbeijmicpdmayeybcedzhicvsfdgrakbaxesjzguqfprcgkgybgzwhxccljpxxjlrjjnddplklqfcsuunt qzbmkqbrhxpasniftpkviphnhfbacfifxkfsjmbgmpzd fanh dous".to_string();
    assert_eq!(Solution::count_anagrams(s), 770563105);

    let s = "smuiquglfwdepzuyqtgujaisius ithsczpelfqp rjm".to_string();
    assert_eq!(Solution::count_anagrams(s), 200923648);

    let s = "b okzojaporykbmq tybq zrztwlolvcyumcsq jjuowpp".to_string();
    assert_eq!(Solution::count_anagrams(s), 210324488);

    assert_eq!(Solution::count_anagrams("too hot".to_string()), 18);
    assert_eq!(Solution::count_anagrams("aa".to_string()), 1);
    assert_eq!(Solution::count_anagrams("a".to_string()), 1);
    assert_eq!(Solution::count_anagrams("a a".to_string()), 1);
    assert_eq!(Solution::count_anagrams("a a a".to_string()), 1);
    assert_eq!(Solution::count_anagrams("a a a a".to_string()), 1);
    assert_eq!(Solution::count_anagrams("a a a a a".to_string()), 1);
    assert_eq!(Solution::count_anagrams("a a a a a a".to_string()), 1);
    assert_eq!(Solution::count_anagrams("a a a a a a a".to_string()), 1);
}
