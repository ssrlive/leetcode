#![allow(dead_code)]

// 3245. Alternating Groups III
// https://leetcode.com/problems/alternating-groups-iii/
// https://leetcode.cn/problems/alternating-groups-iii/
//
// Hard
//
// There are some red and blue tiles arranged circularly. You are given an array of integers colors and a 2D integers array queries.
//
// The color of tile i is represented by colors[i]:
//
//     colors[i] == 0 means that tile i is red.
//     colors[i] == 1 means that tile i is blue.
//
// An alternating group is a contiguous subset of tiles in the circle with alternating colors (each tile in the
// group except the first and last one has a different color from its adjacent tiles in the group).
//
// You have to process queries of two types:
//
//     queries[i] = [1, sizei], determine the count of alternating groups with size sizei.
//     queries[i] = [2, indexi, colori], change colors[indexi] to colori.
//
// Return an array answer containing the results of the queries of the first type in order.
//
// Note that since colors represents a circle, the first and the last tiles are considered to be next to each other.
//
// Example 1:
//
// Input: colors = [0,1,1,0,1], queries = [[2,1,0],[1,4]]
//
// Output: [2]
//
// Explanation:
//
// First query:
//
// Change colors[1] to 0.
//
// Second query:
//
// Count of the alternating groups with size 4:
//
// Example 2:
//
// Input: colors = [0,0,1,0,1,1], queries = [[1,3],[2,3,0],[1,5]]
//
// Output: [2,0]
//
// Explanation:
//
// First query:
//
// Count of the alternating groups with size 3:
//
// Second query: colors will not change.
//
// Third query: There is no alternating group with size 5.
//
// Constraints:
//
//     4 <= colors.length <= 5 * 10^4
//     0 <= colors[i] <= 1
//     1 <= queries.length <= 5 * 10^4
//     queries[i][0] == 1 or queries[i][0] == 2
//     For all i that:
//         queries[i][0] == 1: queries[i].length == 2, 3 <= queries[i][1] <= colors.length - 1
//         queries[i][0] == 2: queries[i].length == 3, 0 <= queries[i][1] <= colors.length - 1, 0 <= queries[i][2] <= 1
//

struct Solution;

impl Solution {
    fn lowbit(x: i32) -> i32 {
        x & -x
    }

    fn update<T>(mut i: i32, x: T, c: &mut [T])
    where
        T: std::ops::AddAssign + Copy,
    {
        if i == 0 {
            return;
        }
        while i < c.len() as i32 {
            c[i as usize] += x;
            i += Self::lowbit(i);
        }
    }

    fn getsum<T>(mut i: i32, c: &[T]) -> T
    where
        T: std::ops::Add<Output = T> + Copy + Default,
    {
        let mut r = T::default();
        while i != 0 {
            r = r + c[i as usize];
            i -= Self::lowbit(i);
        }
        r
    }

    fn insert(n: i32, x: i32, s: &mut std::collections::BTreeSet<i32>, count: &mut [i32], sum: &mut [i64]) {
        s.insert(x);
        let t = s.get(&x).unwrap();
        if s.len() == 1 {
            Self::update(n, 1, count);
            Self::update(n, n as i64, sum);
        } else {
            let before = s.range(..t).next_back().unwrap_or(s.last().unwrap());
            let after = s.range(*t + 1..).next().unwrap_or(s.first().unwrap());

            let len1 = if s.len() == 2 { n } else { (*after - *before + n) % n };
            Self::update(len1, -1, count);
            Self::update(len1, -len1 as i64, sum);
            let len2 = (*t - *before + n) % n;
            Self::update(len2, 1, count);
            Self::update(len2, len2 as i64, sum);
            let len3 = (*after - *t + n) % n;
            Self::update(len3, 1, count);
            Self::update(len3, len3 as i64, sum);
        }
    }

    fn remove(n: i32, x: i32, s: &mut std::collections::BTreeSet<i32>, count: &mut [i32], sum: &mut [i64]) {
        if s.len() == 1 {
            s.clear();
            Self::update(n, -1, count);
            Self::update(n, -n as i64, sum);
        } else {
            let t = s.get(&x).unwrap();
            let before = s.range(..t).next_back().unwrap_or(s.last().unwrap());
            let after = s.range(*t + 1..).next().unwrap_or(s.first().unwrap());
            let len1 = if s.len() == 2 { n } else { (*after - *before + n) % n };
            Self::update(len1, 1, count);
            Self::update(len1, len1 as i64, sum);
            let len2 = (*t - *before + n) % n;
            Self::update(len2, -1, count);
            Self::update(len2, -len2 as i64, sum);
            let len3 = (*after - *t + n) % n;
            Self::update(len3, -1, count);
            Self::update(len3, -len3 as i64, sum);
            s.remove(&x);
        }
    }

    pub fn number_of_alternating_groups(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut colors = colors;
        let n = colors.len() as i32;
        let mut s = std::collections::BTreeSet::new();
        let mut sum = vec![0; n as usize + 1];
        let mut count = vec![0; n as usize + 1];
        let mut r = Vec::new();
        for i in 1..n {
            if colors[i as usize] == colors[i as usize - 1] {
                Self::insert(n, i - 1, &mut s, &mut count, &mut sum);
            }
        }
        if colors[n as usize - 1] == colors[0] {
            Self::insert(n, n - 1, &mut s, &mut count, &mut sum);
        }
        for q in queries {
            if q[0] == 1 {
                let k = q[1];
                r.push(if s.is_empty() {
                    n
                } else {
                    Self::getsum(n, &sum) as i32
                        - Self::getsum(k - 1, &sum) as i32
                        - (k - 1) * (Self::getsum(n, &count) - Self::getsum(k - 1, &count))
                });
            } else {
                let ind = q[1];
                if colors[ind as usize] == q[2] {
                    continue;
                }
                colors[ind as usize] = q[2];
                let before = (ind + n - 1) % n;
                if colors[ind as usize] == colors[before as usize] {
                    Self::insert(n, before, &mut s, &mut count, &mut sum);
                } else {
                    Self::remove(n, before, &mut s, &mut count, &mut sum);
                }
                let after = (ind + 1) % n;
                if colors[ind as usize] == colors[after as usize] {
                    Self::insert(n, ind, &mut s, &mut count, &mut sum);
                } else {
                    Self::remove(n, ind, &mut s, &mut count, &mut sum);
                }
            }
        }
        r
    }
}

/*
impl Solution {
    // This solution can not pass the test cases for performance issue.
    pub fn number_of_alternating_groups(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = colors.len();
        let mut colors = colors.iter().map(|&x| x != 0).collect::<Vec<bool>>();
        colors.extend_from_slice(&colors.clone()[..]);
        let mut blocks = colors.windows(2).map(|x| x[0] != x[1]).collect::<Vec<bool>>();
        let mut result = Vec::new();
        for query in queries {
            if query[0] == 1 {
                let size = query[1] as usize;
                let blkcrop = &blocks[..n + size - 1];
                let diff = blkcrop[1..n + size - 1]
                    .iter()
                    .zip(&blocks[..n + size - 2])
                    .map(|(x, y)| x != y)
                    .collect::<Vec<bool>>();
                let mut breaks = diff
                    .iter()
                    .enumerate()
                    .filter(|&(_, &x)| x)
                    .map(|(i, _)| i + 1)
                    .collect::<Vec<usize>>();
                if blkcrop[0] {
                    breaks.insert(0, 0);
                }
                if *blkcrop.last().unwrap() {
                    breaks.push(n + size - 2);
                }
                let lens = breaks
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .zip(breaks.iter().step_by(2))
                    .map(|(x, y)| x - y + 1)
                    .collect::<Vec<usize>>();
                let count = lens.iter().map(|&x| x as i32 - size as i32 + 1).filter(|&x| x > 0).sum::<i32>();
                result.push(count);
            } else {
                let index = query[1] as usize;
                let color = query[2] == 1;
                for i in (index..colors.len()).step_by(n) {
                    colors[i] = color;
                }
                blocks = colors.windows(2).map(|x| x[0] != x[1]).collect::<Vec<bool>>();
            }
        }
        result
    }
}
*/
#[test]
fn test() {
    let colors = vec![0, 1, 1, 0, 1];
    let queries = vec![vec![2, 1, 0], vec![1, 4]];
    let output = vec![2];
    assert_eq!(Solution::number_of_alternating_groups(colors, queries), output);

    let colors = vec![0, 0, 1, 0, 1, 1];
    let queries = vec![vec![1, 3], vec![2, 3, 0], vec![1, 5]];
    let output = vec![2, 0];
    assert_eq!(Solution::number_of_alternating_groups(colors, queries), output);

    let colors = vec![0, 1, 0, 1];
    let queries = vec![vec![1, 3], vec![2, 2, 1], vec![1, 3], vec![1, 3]];
    let output = vec![4, 1, 1];
    assert_eq!(Solution::number_of_alternating_groups(colors, queries), output);
}
