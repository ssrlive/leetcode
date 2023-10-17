#![allow(dead_code)]

// 2813. Maximum Elegance of a K-Length Subsequence
// https://leetcode.com/problems/maximum-elegance-of-a-k-length-subsequence/
// https://leetcode.cn/problems/maximum-elegance-of-a-k-length-subsequence/
//
// Hard
//
// You are given a 0-indexed 2D integer array items of length n and an integer k.
//
// items[i] = [profiti, categoryi], where profiti and categoryi denote the profit and category of the ith item respectively.
//
// Let's define the elegance of a subsequence of items as total_profit + distinct_categories2,
// where total_profit is the sum of all profits in the subsequence, and distinct_categories is
// the number of distinct categories from all the categories in the selected subsequence.
//
// Your task is to find the maximum elegance from all subsequences of size k in items.
//
// Return an integer denoting the maximum elegance of a subsequence of items with size exactly k.
//
// Note: A subsequence of an array is a new array generated from the original array by deleting
// some elements (possibly none) without changing the remaining elements' relative order.
//
// Example 1:
//
// Input: items = [[3,2],[5,1],[10,1]], k = 2
// Output: 17
// Explanation: In this example, we have to select a subsequence of size 2.
// We can select items[0] = [3,2] and items[2] = [10,1].
// The total profit in this subsequence is 3 + 10 = 13, and the subsequence contains 2 distinct categories [2,1].
// Hence, the elegance is 13 + 22 = 17, and we can show that it is the maximum achievable elegance.
//
// Example 2:
//
// Input: items = [[3,1],[3,1],[2,2],[5,3]], k = 3
// Output: 19
// Explanation: In this example, we have to select a subsequence of size 3.
// We can select items[0] = [3,1], items[2] = [2,2], and items[3] = [5,3].
// The total profit in this subsequence is 3 + 2 + 5 = 10, and the subsequence contains 3 distinct categories [1,2,3].
// Hence, the elegance is 10 + 32 = 19, and we can show that it is the maximum achievable elegance.
//
// Example 3:
//
// Input: items = [[1,1],[2,1],[3,1]], k = 3
// Output: 7
// Explanation: In this example, we have to select a subsequence of size 3.
// We should select all the items.
// The total profit will be 1 + 2 + 3 = 6, and the subsequence contains 1 distinct category [1].
// Hence, the maximum elegance is 6 + 12 = 7.
//
// Constraints:
//
// 1 <= items.length == n <= 10^5
// items[i].length == 2
// items[i][0] == profiti
// items[i][1] == categoryi
// 1 <= profiti <= 10^9
// 1 <= categoryi <= n
// 1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn find_maximum_elegance(items: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut items = items
            .iter()
            .map(|v| v.iter().map(|&x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let k = k as usize;
        items.sort_unstable_by(|a, b| b[0].cmp(&a[0]));
        let mut vis = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();
        let mut tot = 0;
        for item in items.iter().take(k) {
            if vis.contains(&item[1]) {
                heap.push(std::cmp::Reverse(item[0]));
            } else {
                vis.insert(item[1]);
            }
            tot += item[0];
        }
        let mut ans = tot + vis.len() as i64 * vis.len() as i64;
        for item in items.iter().skip(k) {
            if !heap.is_empty() && !vis.contains(&item[1]) {
                vis.insert(item[1]);
                tot -= heap.pop().unwrap().0;
                tot += item[0];
                ans = ans.max(tot + vis.len() as i64 * vis.len() as i64);
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_maximum_elegance(vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2), 17);

    let v = vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]];
    assert_eq!(Solution::find_maximum_elegance(v, 3), 19);

    assert_eq!(Solution::find_maximum_elegance(vec![vec![1, 1], vec![2, 1], vec![3, 1]], 3), 7);

    let v = vec![
        vec![6, 4],
        vec![14, 4],
        vec![14, 6],
        vec![15, 6],
        vec![2, 10],
        vec![4, 1],
        vec![3, 2],
        vec![13, 4],
        vec![3, 3],
        vec![12, 5],
    ];
    assert_eq!(Solution::find_maximum_elegance(v, 6), 88);
}
