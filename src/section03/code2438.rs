#![allow(dead_code)]

// 2438. Range Product Queries of Powers
// https://leetcode.com/problems/range-product-queries-of-powers/
// https://leetcode.cn/problems/range-product-queries-of-powers/
//
// Given a positive integer n, there exists a 0-indexed array called powers, composed of the minimum number of powers of 2 that sum to n.
// The array is sorted in non-decreasing order, and there is only one way to form the array.
//
// You are also given a 0-indexed 2D integer array queries, where queries[i] = [lefti, righti]. Each queries[i] represents a query
// where you have to find the product of all powers[j] with lefti <= j <= righti.
//
// Return an array answers, equal in length to queries, where answers[i] is the answer to the ith query.
// Since the answer to the ith query may be too large, each answers[i] should be returned modulo 109 + 7.
//
// Example 1:
//
// Input: n = 15, queries = [[0,1],[2,2],[0,3]]
// Output: [2,4,64]
// Explanation:
// For n = 15, powers = [1,2,4,8]. It can be shown that powers cannot be a smaller size.
// Answer to 1st query: powers[0] * powers[1] = 1 * 2 = 2.
// Answer to 2nd query: powers[2] = 4.
// Answer to 3rd query: powers[0] * powers[1] * powers[2] * powers[3] = 1 * 2 * 4 * 8 = 64.
// Each answer modulo 109 + 7 yields the same answer, so [2,4,64] is returned.
//
// Example 2:
//
// Input: n = 2, queries = [[0,0]]
// Output: [2]
// Explanation:
// For n = 2, powers = [2].
// The answer to the only query is powers[0] = 2. The answer modulo 109 + 7 is the same, so [2] is returned.
//
// Constraints:
//
// - 1 <= n <= 10^9
// - 1 <= queries.length <= 10^5
// - 0 <= starti <= endi < powers.length
//

struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const Q: i64 = 1_000_000_007;
        let mut n = n as i64;
        let mut powers = vec![];
        let mut exp = 0;
        while n > 0 {
            if n % 2 == 1 {
                powers.push(1 << exp);
            }
            exp += 1;
            n >>= 1;
        }

        let m = powers.len();
        let mut lookup = vec![vec![0; m]; m];
        for i in 0..m {
            lookup[i][i] = powers[i];
            for (j, &item) in powers.iter().enumerate().skip(i + 1) {
                lookup[i][j] = ((item as i64 * lookup[i][j - 1] as i64) % Q) as i32;
            }
        }
        queries.into_iter().map(|v| lookup[v[0] as usize][v[1] as usize]).collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (15, vec![vec![0, 1], vec![2, 2], vec![0, 3]], vec![2, 4, 64]),
        (2, vec![vec![0, 0]], vec![2]),
    ];

    for (n, queries, expected) in cases {
        assert_eq!(Solution::product_queries(n, queries), expected);
    }
}
