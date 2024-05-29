#![allow(dead_code)]

// 3160. Find the Number of Distinct Colors Among the Balls
// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
// https://leetcode.cn/problems/find-the-number-of-distinct-colors-among-the-balls/
//
// Medium
//
// You are given an integer limit and a 2D array queries of size n x 2.
//
// There are limit + 1 balls with distinct labels in the range [0, limit]. Initially, all balls are uncolored. For every query in queries that is of the form [x, y], you mark ball x with the color y. After each query, you need to find the number of distinct colors among the balls.
//
// Return an array result of length n, where result[i] denotes the number of distinct colors after ith query.
//
// Note that when answering a query, lack of a color will not be considered as a color.
//
// Example 1:
//
// Input: limit = 4, queries = [[1,4],[2,5],[1,3],[3,4]]
//
// Output: [1,2,2,3]
//
// Explanation:
//
// After query 0, ball 1 has color 4.
// After query 1, ball 1 has color 4, and ball 2 has color 5.
// After query 2, ball 1 has color 3, and ball 2 has color 5.
// After query 3, ball 1 has color 3, ball 2 has color 5, and ball 3 has color 4.
//
// Example 2:
//
// Input: limit = 4, queries = [[0,1],[1,2],[2,2],[3,4],[4,5]]
//
// Output: [1,2,2,3,4]
//
// Explanation:
//
// After query 0, ball 0 has color 1.
// After query 1, ball 0 has color 1, and ball 1 has color 2.
// After query 2, ball 0 has color 1, and balls 1 and 2 have color 2.
// After query 3, ball 0 has color 1, balls 1 and 2 have color 2, and ball 3 has color 4.
// After query 4, ball 0 has color 1, balls 1 and 2 have color 2, ball 3 has color 4, and ball 4 has color 5.
//
// Constraints:
//
// 1 <= limit <= 10^9
// 1 <= n == queries.length <= 10^5
// queries[i].length == 2
// 0 <= queries[i][0] <= limit
// 1 <= queries[i][1] <= 10^9
//

struct Solution;

impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        queries
            .into_iter()
            .fold((HashMap::new(), HashMap::new(), vec![]), |(mut m, mut s, mut v), x| {
                if let std::collections::hash_map::Entry::Vacant(e) = m.entry(x[0]) {
                    e.insert(x[1]);
                    *s.entry(x[1]).or_insert(0) += 1;
                } else {
                    let col = m.get(&x[0]).unwrap();
                    let val = s.get(col).unwrap();

                    if *val <= 1 {
                        s.remove(col);
                    } else {
                        *s.entry(*col).or_insert(0) -= 1;
                    }

                    *m.entry(x[0]).or_insert(-1) = x[1];
                    *s.entry(x[1]).or_insert(0) += 1;
                }
                v.push(s.len() as i32);
                (m, s, v)
            })
            .2
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]),
        vec![1, 2, 2, 3]
    );
    assert_eq!(
        Solution::query_results(4, vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]),
        vec![1, 2, 2, 3, 4]
    );
}
