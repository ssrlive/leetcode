#![allow(dead_code)]

/*

// 2217. Find Palindrome With Fixed Length
// https://leetcode.com/problems/find-palindrome-with-fixed-length/
// https://leetcode.cn/problems/find-palindrome-with-fixed-length/
//
// Medium
//
// Given an integer array queries and a positive integer intLength, return an array answer where answer[i] is either the queries[i]th smallest positive palindrome of length intLength or -1 if no such palindrome exists.

A palindrome is a number that reads the same backwards and forwards. Palindromes cannot have leading zeros.

Example 1:

Input: queries = [1,2,3,4,5,90], intLength = 3
Output: [101,111,121,131,141,999]
Explanation:
The first few palindromes of length 3 are:
101, 111, 121, 131, 141, 151, 161, 171, 181, 191, 202, ...
The 90th palindrome of length 3 is 999.

Example 2:

Input: queries = [2,4,6], intLength = 4
Output: [1111,1331,1551]
Explanation:
The first six palindromes of length 4 are:
1001, 1111, 1221, 1331, 1441, and 1551.

Constraints:

    1 <= queries.length <= 5 * 10^4
    1 <= queries[i] <= 10^9
    1 <= intLength <= 15
*/

struct Solution;

impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        fn generate_palindromes(pos: i32, len: u32) -> i64 {
            if len % 2 == 0 {
                let left_half = (pos - 1) + 10_i32.pow((len - 1) / 2);
                format!(
                    "{}{}",
                    left_half,
                    left_half.to_string().chars().rev().collect::<String>()
                )
            } else {
                let left_half = (pos - 1) + 10_i32.pow((len - 1) / 2);
                format!(
                    "{}{}",
                    left_half,
                    &left_half.to_string().chars().rev().collect::<String>()[1..]
                )
            }
            .parse::<i64>()
            .unwrap()
        }

        let mut palindromes = Vec::new();
        let max_palindrome_count = 9 * 10_i32.pow((int_length as u32 - 1) / 2);
        let query_range = 1..=max_palindrome_count;
        for query in queries {
            if query_range.contains(&query) {
                palindromes.push(generate_palindromes(query, int_length as u32));
            } else {
                palindromes.push(-1);
            }
        }
        palindromes
    }
}

#[test]
fn test() {}
