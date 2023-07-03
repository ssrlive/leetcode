#![allow(dead_code)]

/*

// 1923. Longest Common Subpath
// https://leetcode.com/problems/longest-common-subpath/
// https://leetcode.cn/problems/longest-common-subpath/
//
// Hard
//
// There is a country of n cities numbered from 0 to n - 1. In this country, there is a road connecting every pair of cities.

There are m friends numbered from 0 to m - 1 who are traveling through the country. Each one of them will take a path consisting of some cities. Each path is represented by an integer array that contains the visited cities in order. The path may contain a city more than once, but the same city will not be listed consecutively.

Given an integer n and a 2D integer array paths where paths[i] is an integer array representing the path of the ith friend, return the length of the longest common subpath that is shared by every friend's path, or 0 if there is no common subpath at all.

A subpath of a path is a contiguous sequence of cities within that path.

Example 1:

Input: n = 5, paths = [[0,1,2,3,4],
                       [2,3,4],
                       [4,0,1,2,3]]
Output: 2
Explanation: The longest common subpath is [2,3].

Example 2:

Input: n = 3, paths = [[0],[1],[2]]
Output: 0
Explanation: There is no common subpath shared by the three paths.

Example 3:

Input: n = 5, paths = [[0,1,2,3,4],
                       [4,3,2,1,0]]
Output: 1
Explanation: The possible longest common subpaths are [0], [1], [2], [3], and [4]. All have a length of 1.

Constraints:

    1 <= n <= 10^5
    m == paths.length
    2 <= m <= 10^5
    sum(paths[i].length) <= 10^5
    0 <= paths[i][j] < n
    The same city is not listed multiple times consecutively in paths[i].
*/

struct Solution;

impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        fn check_has_common_subpath_with_length_n(n: usize, base: i64, paths: &[Vec<i32>]) -> bool {
            if n == 0 {
                return true;
            }
            // NOTE: rabin-karp
            let modulo = 1_000_000_009;
            let base_pow_n = {
                let mut res = 1;
                let mut n = n;
                while n > 1 {
                    res = (res * base) % modulo;
                    n -= 1;
                }
                res
            };

            let mut hm: HashMap<i64, Vec<usize>> = HashMap::new();
            for (path_idx, path) in paths.iter().enumerate() {
                let mut new_hm = HashMap::new();
                let mut hash = {
                    let mut res = 0;
                    for &path_i in path.iter().take(n) {
                        res = (res * base + path_i as i64) % modulo;
                    }
                    res
                };
                for i in 0..=(path.len() - n) {
                    if path_idx == 0 {
                        new_hm.entry(hash).or_insert(vec![]).push(i);
                    } else {
                        let prev_path_idx = path_idx - 1;
                        if let Some(prev_matches) = hm.get(&hash) {
                            let matches = prev_matches
                                .iter()
                                .any(|&prev_i| paths[prev_path_idx][prev_i..(prev_i + n)] == path[i..(i + n)]);
                            if matches {
                                new_hm.entry(hash).or_insert(vec![]).push(i);
                            }
                        }
                    }
                    if i < path.len() - n {
                        hash -= path[i] as i64 * base_pow_n;
                        hash *= base;
                        hash += path[i + n] as i64;
                        hash %= modulo;
                        if hash < 0 {
                            hash += modulo;
                        }
                    }
                }
                hm = new_hm;
            }

            !hm.is_empty()
        }

        let n = n as i64;
        let upper_bound = paths.iter().map(|a| a.len()).min().unwrap();

        let mut l = 0;
        let mut r = upper_bound;
        while l < r {
            let mid = l + (r - l) / 2;
            let has = check_has_common_subpath_with_length_n(mid, n, &paths);
            if has {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if l == upper_bound && check_has_common_subpath_with_length_n(l, n, &paths) {
            return l as i32;
        }
        l as i32 - 1
    }
}

#[test]
fn test() {
    let cases = vec![
        (5, vec![vec![0, 1, 2, 3, 4], vec![2, 3, 4], vec![4, 0, 1, 2, 3]], 2),
        (3, vec![vec![0], vec![1], vec![2]], 0),
        (5, vec![vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1, 0]], 1),
        (5, vec![vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4]], 1),
    ];
    for (n, paths, expected) in cases {
        assert_eq!(Solution::longest_common_subpath(n, paths), expected);
    }
}
