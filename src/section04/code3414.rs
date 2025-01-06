#![allow(dead_code)]

// 3414. Maximum Score of Non-overlapping Intervals
// https://leetcode.com/problems/maximum-score-of-non-overlapping-intervals/
// https://leetcode.cn/problems/maximum-score-of-non-overlapping-intervals/
//
// Hard
//
// You are given a 2D integer array intervals, where intervals[i] = [li, ri, weighti]. Interval i starts at position
// li and ends at ri, and has a weight of weighti. You can choose up to 4 non-overlapping intervals.
// The score of the chosen intervals is defined as the total sum of their weights.
//
// Return the lexicographically smallest array of at most 4 indices from intervals with maximum score,
// representing your choice of non-overlapping intervals.
//
// Two intervals are said to be non-overlapping if they do not share any points.
// In particular, intervals sharing a left or right boundary are considered overlapping.
//
// An array a is lexicographically smaller than an array b if in the first position where a and b differ,
// array a has an element that is less than the corresponding element in b.
// If the first min(a.length, b.length) elements do not differ, then the shorter array is the lexicographically smaller one.
//
// Example 1:
//
// Input: intervals = [[1,3,2],[4,5,2],[1,5,5],[6,9,3],[6,7,1],[8,9,1]]
//
// Output: [2,3]
//
// Explanation:
//
// You can choose the intervals with indices 2, and 3 with respective weights of 5, and 3.
//
// Example 2:
//
// Input: intervals = [[5,8,1],[6,7,7],[4,7,3],[9,10,6],[7,8,2],[11,14,3],[3,5,5]]
//
// Output: [1,3,5,6]
//
// Explanation:
//
// You can choose the intervals with indices 1, 3, 5, and 6 with respective weights of 7, 6, 3, and 5.
//
// Constraints:
//
//     1 <= intevals.length <= 5 * 10^4
//     intervals[i].length == 3
//     intervals[i] = [li, ri, weighti]
//     1 <= li <= ri <= 10^9
//     1 <= weighti <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::BTreeMap;
        fn solve(v: &Vec<Vec<i32>>, taken: i32, idx: usize, dp: &mut BTreeMap<(usize, i32), (i64, Vec<i32>)>) -> (i64, Vec<i32>) {
            let n = v.len();
            if idx >= n || taken == 4 {
                return (0, vec![]);
            }
            if let Some(ans) = dp.get(&(idx, taken)) {
                return ans.clone();
            }
            let ans1 = solve(v, taken, idx + 1, dp);
            let mut next_idx = n;
            let mut left = idx;
            let mut right = n - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if v[mid][0] > v[idx][1] {
                    next_idx = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            let mut ans2 = solve(v, taken + 1, next_idx, dp);
            ans2.0 += v[idx][2] as i64;
            ans2.1.push(v[idx][3]);
            ans2.1.sort();

            let ans = match ans1.0.cmp(&ans2.0) {
                std::cmp::Ordering::Greater => ans1,
                std::cmp::Ordering::Less => ans2,
                std::cmp::Ordering::Equal => (ans1.0, ans1.1.min(ans2.1)),
            };

            dp.insert((idx, taken), ans.clone());
            ans
        }

        let mut dp = BTreeMap::new();
        let mut v = vec![];
        for (i, intervals_i) in intervals.iter().enumerate() {
            let l = intervals_i[0];
            let r = intervals_i[1];
            let wt = intervals_i[2];
            v.push(vec![l, r, wt, i as i32]);
        }
        v.sort();
        solve(&v, 0, 0, &mut dp).1
    }
}

#[test]
fn test() {
    let intervals = vec![
        vec![1, 3, 2],
        vec![4, 5, 2],
        vec![1, 5, 5],
        vec![6, 9, 3],
        vec![6, 7, 1],
        vec![8, 9, 1],
    ];
    let output = vec![2, 3];
    assert_eq!(Solution::maximum_weight(intervals), output);

    let intervals = vec![
        vec![5, 8, 1],
        vec![6, 7, 7],
        vec![4, 7, 3],
        vec![9, 10, 6],
        vec![7, 8, 2],
        vec![11, 14, 3],
        vec![3, 5, 5],
    ];
    let output = vec![1, 3, 5, 6];
    assert_eq!(Solution::maximum_weight(intervals), output);
}
