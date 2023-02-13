#![allow(dead_code)]

/*

// 2564. Substring XOR Queries
// https://leetcode.com/problems/substring-xor-queries/
// https://leetcode.cn/problems/substring-xor-queries/
//
// Medium
//
// You are given a binary string s, and a 2D integer array queries where queries[i] = [firsti, secondi].

For the ith query, find the shortest substring of s whose decimal value, val, yields secondi when bitwise XORed with firsti. In other words, val ^ firsti == secondi.

The answer to the ith query is the endpoints (0-indexed) of the substring [lefti, righti] or [-1, -1] if no such substring exists. If there are multiple answers, choose the one with the minimum lefti.

Return an array ans where ans[i] = [lefti, righti] is the answer to the ith query.

A substring is a contiguous non-empty sequence of characters within a string.

Example 1:

Input: s = "101101", queries = [[0,5],[1,2]]
Output: [[0,2],[2,3]]
Explanation: For the first query the substring in range [0,2] is "101" which has a decimal value of 5, and 5 ^ 0 = 5, hence the answer to the first query is [0,2]. In the second query, the substring in range [2,3] is "11", and has a decimal value of 3, and 3 ^ 1 = 2. So, [2,3] is returned for the second query.

Example 2:

Input: s = "0101", queries = [[12,8]]
Output: [[-1,-1]]
Explanation: In this example there is no substring that answers the query, hence [-1,-1] is returned.

Example 3:

Input: s = "1", queries = [[4,5]]
Output: [[0,0]]
Explanation: For this example, the substring in range [0,0] has a decimal value of 1, and 1 ^ 4 = 5. So, the answer is [0,0].

Constraints:

    1 <= s.length <= 10^4
    s[i] is either '0' or '1'.
    1 <= queries.length <= 10^5
    0 <= firsti, secondi <= 10^9
*/

struct Solution;

impl Solution {
    pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut mp = HashMap::<i32, (i32, i32)>::new();

        for i in 0..n {
            let mut val = 0;
            for j in 0..32 {
                if j + i == n {
                    break;
                }
                if j > 0 && s[i] == '0' {
                    break;
                }

                val = 2 * val + (s[i + j] as u8 - b'0') as i32;
                if mp.contains_key(&val) {
                    continue;
                }

                mp.insert(val, (i as i32, i as i32 + j as i32));
            }
        }

        let m = queries.len();
        let mut ret = vec![vec![-1, -1]; m];

        for i in 0..m {
            if let Some(p) = mp.get(&(queries[i][0] ^ queries[i][1])) {
                ret[i] = vec![p.0, p.1];
            }
        }

        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            "101101".to_string(),
            vec![vec![0, 5], vec![1, 2]],
            vec![vec![0, 2], vec![2, 3]],
        ),
        ("0101".to_string(), vec![vec![12, 8]], vec![vec![-1, -1]]),
        ("1".to_string(), vec![vec![4, 5]], vec![vec![0, 0]]),
    ];
    for (s, queries, expect) in cases {
        assert_eq!(Solution::substring_xor_queries(s, queries), expect);
    }
}
