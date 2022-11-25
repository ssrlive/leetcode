#![allow(dead_code)]

// 399. Evaluate Division
// https://leetcode.com/problems/evaluate-division/
//
// You are given an array of variable pairs equations and an array of real numbers values, where equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.
//
// You are also given some queries, where queries[j] = [Cj, Dj] represents the jth query where you must find the answer for Cj / Dj = ?.
//
// Return the answers to all queries. If a single answer cannot be determined, return -1.0.
//
// Note: The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.
//
// Example 1:
//
// Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
// Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
// Explanation:
// Given: a / b = 2.0, b / c = 3.0
// queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
// return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
//
// Example 2:
//
// Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
// Output: [3.75000,0.40000,5.00000,0.20000]
//
// Example 3:
//
// Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
// Output: [0.50000,2.00000,-1.00000,-1.00000]
//
// Constraints:
//
// - 1 <= equations.length <= 20
// - equations[i].length == 2
// - 1 <= Ai.length, Bi.length <= 5
// - values.length == equations.length
// - 0.0 < values[i] <= 20.0
// - 1 <= queries.length <= 20
// - queries[i].length == 2
// - 1 <= Cj.length, Dj.length <= 5
// - Ai, Bi, Cj, Dj consist of lower case English letters and digits.

struct Solution;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut m = std::collections::HashMap::<String, Vec<(String, f64)>>::new();
        for (i, equation) in equations.iter().enumerate() {
            let (a, b) = (&equation[0], &equation[1]);
            m.entry(a.clone()).or_default().push((b.clone(), values[i]));
            m.entry(b.clone()).or_default().push((a.clone(), 1.0 / values[i]));
        }
        let mut result = Vec::new();
        for query in queries {
            let (a, b) = (&query[0], &query[1]);
            if !m.contains_key(a) || !m.contains_key(b) {
                result.push(-1.0);
                continue;
            }
            let mut visited = std::collections::HashSet::<String>::new();
            let mut stack = Vec::new();
            stack.push((a.clone(), 1.0));
            let mut found = false;
            while let Some((node, value)) = stack.pop() {
                if node == *b {
                    result.push(value);
                    found = true;
                    break;
                }
                visited.insert(node.clone());
                for (next, next_value) in m.get(&node).unwrap() {
                    if !visited.contains(next) {
                        stack.push((next.clone(), value * next_value));
                    }
                }
            }
            if !found {
                result.push(-1.0);
            }
        }
        result
    }
}

#[test]
fn test() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];
    let result = vec![6.0, 0.5, -1.0, 1.0, -1.0];
    assert_eq!(Solution::calc_equation(equations, values, queries), result);
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
        vec!["bc".to_string(), "cd".to_string()],
    ];
    let values = vec![1.5, 2.5, 5.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["c".to_string(), "b".to_string()],
        vec!["bc".to_string(), "cd".to_string()],
        vec!["cd".to_string(), "bc".to_string()],
    ];
    let result = vec![3.75, 0.4, 5.0, 0.2];
    assert_eq!(Solution::calc_equation(equations, values, queries), result);
    let equations = vec![vec!["a".to_string(), "b".to_string()]];
    let values = vec![0.5];
    let queries = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "c".to_string()],
        vec!["x".to_string(), "y".to_string()],
    ];
    let result = vec![0.5, 2.0, -1.0, -1.0];
    assert_eq!(Solution::calc_equation(equations, values, queries), result);
}
