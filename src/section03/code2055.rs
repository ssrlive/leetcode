#![allow(dead_code)]

/*

// 2055. Plates Between Candles
// https://leetcode.com/problems/plates-between-candles/
// https://leetcode.cn/problems/plates-between-candles/
//
// Medium
//
// There is a long table with a line of plates and candles arranged on top of it. You are given a 0-indexed string s consisting of characters '*' and '|' only, where a '*' represents a plate and a '|' represents a candle.

You are also given a 0-indexed 2D integer array queries where queries[i] = [lefti, righti] denotes the substring s[lefti...righti] (inclusive). For each query, you need to find the number of plates between candles that are in the substring. A plate is considered between candles if there is at least one candle to its left and at least one candle to its right in the substring.

    For example, s = "||**||**|*", and a query [3, 8] denotes the substring "*||**|". The number of plates between candles in this substring is 2, as each of the two plates has at least one candle in the substring to its left and right.

Return an integer array answer where answer[i] is the answer to the ith query.

Example 1:
ex-1

Input: s = "**|**|***|", queries = [[2,5],[5,9]]
Output: [2,3]
Explanation:
- queries[0] has two plates between candles.
- queries[1] has three plates between candles.

Example 2:
ex-2

Input: s = "***|**|*****|**||**|*", queries = [[1,17],[4,5],[14,17],[5,11],[15,16]]
Output: [9,0,0,0,0]
Explanation:
- queries[0] has nine plates between candles.
- The other queries have zero plates between candles.

Constraints:

    3 <= s.length <= 10^5
    s consists of '*' and '|' characters.
    1 <= queries.length <= 10^5
    queries[i].length == 2
    0 <= lefti <= righti < s.length
*/

struct Solution;

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![0; queries.len()];
        let pos: Vec<_> = s
            .chars()
            .enumerate()
            .filter(|&(_, ch)| ch == '|')
            .map(|(idx, _)| idx as i32)
            .collect();
        for (idx, query) in queries.iter().enumerate() {
            let left = query[0];
            let right = query[1];
            let leftmost = pos.binary_search(&left).unwrap_or_else(|e| e) as i32;
            let rightmost = pos.binary_search(&right).map_or_else(|e| e as i32 - 1, |v| v as i32);
            if leftmost < rightmost {
                res[idx] = pos[rightmost as usize] - pos[leftmost as usize] - (rightmost - leftmost);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("**|**|***|", vec![vec![2, 5], vec![5, 9]], vec![2, 3]),
        (
            "***|**|*****|**||**|*",
            vec![vec![1, 17], vec![4, 5], vec![14, 17], vec![5, 11], vec![15, 16]],
            vec![9, 0, 0, 0, 0],
        ),
    ];
    for (s, queries, expect) in cases {
        assert_eq!(Solution::plates_between_candles(s.to_string(), queries), expect);
    }
}
