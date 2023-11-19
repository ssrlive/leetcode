#![allow(dead_code)]

// 2940. Find Building Where Alice and Bob Can Meet
// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/
// https://leetcode.cn/problems/find-building-where-alice-and-bob-can-meet/
//
// Hard
//
// You are given a 0-indexed array heights of positive integers, where heights[i] represents the height of the ith building.
//
// If a person is in building i, they can move to any other building j if and only if i < j and heights[i] < heights[j].
//
// You are also given another array queries where queries[i] = [ai, bi]. On the ith query, Alice is in building ai while Bob is in building bi.
//
// Return an array ans where ans[i] is the index of the leftmost building where Alice and Bob can meet on the ith query.
// If Alice and Bob cannot move to a common building on query i, set ans[i] to -1.
//
// Example 1:
//
// Input: heights = [6,4,8,5,2,7], queries = [[0,1],[0,3],[2,4],[3,4],[2,2]]
// Output: [2,5,-1,5,2]
// Explanation: In the first query, Alice and Bob can move to building 2 since heights[0] < heights[2] and heights[1] < heights[2].
// In the second query, Alice and Bob can move to building 5 since heights[0] < heights[5] and heights[3] < heights[5].
// In the third query, Alice cannot meet Bob since Alice cannot move to any other building.
// In the fourth query, Alice and Bob can move to building 5 since heights[3] < heights[5] and heights[4] < heights[5].
// In the fifth query, Alice and Bob are already in the same building.
// For ans[i] != -1, It can be shown that ans[i] is the leftmost building where Alice and Bob can meet.
// For ans[i] == -1, It can be shown that there is no building where Alice and Bob can meet.
//
// Example 2:
//
// Input: heights = [5,3,8,2,6,1,4,6], queries = [[0,7],[3,5],[5,2],[3,0],[1,6]]
// Output: [7,6,-1,4,6]
// Explanation: In the first query, Alice can directly move to Bob's building since heights[0] < heights[7].
// In the second query, Alice and Bob can move to building 6 since heights[3] < heights[6] and heights[5] < heights[6].
// In the third query, Alice cannot meet Bob since Bob cannot move to any other building.
// In the fourth query, Alice and Bob can move to building 4 since heights[3] < heights[4] and heights[0] < heights[4].
// In the fifth query, Alice can directly move to Bob's building since heights[1] < heights[6].
// For ans[i] != -1, It can be shown that ans[i] is the leftmost building where Alice and Bob can meet.
// For ans[i] == -1, It can be shown that there is no building where Alice and Bob can meet.
//
// Constraints:
//
//     1 <= heights.length <= 5 * 10^4
//     1 <= heights[i] <= 10^9
//     1 <= queries.length <= 5 * 10^4
//     queries[i] = [ai, bi]
//     0 <= ai, bi <= heights.length - 1
//

struct Solution;

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pq = std::collections::BinaryHeap::new();
        let (n, m) = (heights.len(), queries.len());

        for (i, &height) in heights.iter().enumerate().take(n) {
            pq.push((height, i));
        }

        let mut a = vec![];

        for (i, query) in queries.iter().enumerate().take(m) {
            let (u, v) = (query[0] as usize, query[1] as usize);
            a.push((heights[u].max(heights[v]), i));
        }
        a.sort();

        let mut s = std::collections::BTreeSet::new();
        let mut ret = vec![0; m];

        while let Some((h, k)) = a.pop() {
            while !pq.is_empty() && pq.peek().unwrap().0 > h {
                s.insert(pq.peek().unwrap().1);
                pq.pop();
            }

            let (u, v) = (queries[k][0] as usize, queries[k][1] as usize);
            let (mn, mx) = (u.min(v), u.max(v));

            if mn == mx || heights[mn] < heights[mx] {
                ret[k] = mx as i32;
                continue;
            }

            if let Some(j) = s.range(mx + 1..).next() {
                ret[k] = (*j) as i32;
            } else {
                ret[k] = -1;
            }
        }

        ret
    }
}

#[test]
fn test() {
    let heights = vec![6, 4, 8, 5, 2, 7];
    let queries = vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]];
    let ans = vec![2, 5, -1, 5, 2];
    assert_eq!(Solution::leftmost_building_queries(heights, queries), ans);

    let heights = vec![5, 3, 8, 2, 6, 1, 4, 6];
    let queries = vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]];
    let ans = vec![7, 6, -1, 4, 6];
    assert_eq!(Solution::leftmost_building_queries(heights, queries), ans);
}
