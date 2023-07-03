#![allow(dead_code)]

/*

// 1573. Number of Ways to Split a String
Medium
572
71
Companies

Given a binary string s, you can split s into 3 non-empty strings s1, s2, and s3 where s1 + s2 + s3 = s.

Return the number of ways s can be split such that the number of ones is the same in s1, s2, and s3. Since the answer may be too large, return it modulo 109 + 7.

Example 1:

Input: s = "10101"
Output: 4
Explanation: There are four ways to split s in 3 parts where each part contain the same number of letters '1'.
"1|010|1"
"1|01|01"
"10|10|1"
"10|1|01"

Example 2:

Input: s = "1001"
Output: 0

Example 3:

Input: s = "0000"
Output: 3
Explanation: There are three ways to split s in 3 parts.
"0|0|00"
"0|00|0"
"00|0|0"

Constraints:

    3 <= s.length <= 10^5
    s[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut one = 0;
        let mod_num = 1_000_000_007;
        let ans;
        for &item in s.iter() {
            if item == b'1' {
                one += 1;
            }
        }
        if one % 3 != 0 {
            return 0;
        } else if one == 0 {
            ans = ((n as i64 - 1) * (n as i64 - 2) / 2) % mod_num;
        } else {
            let count_of_one = one / 3;
            let (mut index1, mut index1end, mut index2, mut index2end) = (-1, -1, -1, -1);
            let mut count = 0;
            for (i, &item) in s.iter().enumerate() {
                if item == b'1' {
                    count += 1;
                }
                if count == count_of_one && index1 == -1 {
                    index1 = i as i64;
                }
                if count == count_of_one + 1 && index2 == -1 {
                    index2 = i as i64;
                }
                if count == count_of_one * 2 && index1end == -1 {
                    index1end = i as i64;
                }
                if count == count_of_one * 2 + 1 && index2end == -1 {
                    index2end = i as i64;
                }
            }
            ans = ((index2 - index1) % mod_num) * ((index2end - index1end) % mod_num) % mod_num;
        }

        ans as _
    }
}

#[test]
fn test() {
    let cases = vec![("10101".to_string(), 4), ("1001".to_string(), 0), ("0000".to_string(), 3)];
    for (s, expected) in cases {
        assert_eq!(Solution::num_ways(s), expected);
    }
}
