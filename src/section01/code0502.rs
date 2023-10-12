#![allow(dead_code)]

// 502. IPO
// https://leetcode.com/problems/ipo/
// https://leetcode.cn/problems/ipo/
//
// Suppose LeetCode will start its IPO soon. In order to sell a good price of its shares to Venture Capital, LeetCode would like to work on some projects to increase its capital before the IPO. Since it has limited resources, it can only finish at most k distinct projects before the IPO. Help LeetCode design the best way to maximize its total capital after finishing at most k distinct projects.
//
// You are given n projects where the ith project has a pure profit profits[i] and a minimum capital of capital[i] is needed to start it.
//
// Initially, you have w capital. When you finish a project, you will obtain its pure profit and the profit will be added to your total capital.
//
// Pick a list of at most k distinct projects from given projects to maximize your final capital, and return the final maximized capital.
//
// The answer is guaranteed to fit in a 32-bit signed integer.
//
//
// Example 1:
//
// Input: k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
// Output: 4
// Explanation: Since your initial capital is 0, you can only start the project indexed 0.
// After finishing it you will obtain profit 1 and your capital becomes 1.
// With capital 1, you can either start the project indexed 1 or the project indexed 2.
// Since you can choose at most 2 projects, you need to finish the project indexed 2 to get the maximum capital.
// Therefore, output the final maximized capital, which is 0 + 1 + 3 = 4.
//
// Example 2:
//
// Input: k = 3, w = 0, profits = [1,2,3], capital = [0,1,2]
// Output: 6
//
// Constraints:
//
// - 1 <= k <= 10^5
// - 0 <= w <= 10^9
// - n == profits.length
// - n == capital.length
// - 1 <= n <= 10^5
// - 0 <= profits[i] <= 10^4
// - 0 <= capital[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn find_maximized_capital3(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects = capital.into_iter().zip(profits).collect::<Vec<_>>();
        projects.sort_by_key(|&(_, c)| c);
        let mut heap = std::collections::BinaryHeap::new();
        let mut i = 0;
        let mut w = w;
        for _ in 0..k {
            while i < projects.len() && projects[i].1 <= w {
                heap.push(std::cmp::Reverse(projects[i].0));
                i += 1;
            }
            if let Some(std::cmp::Reverse(p)) = heap.pop() {
                w += p;
            } else {
                break;
            }
        }
        w
    }

    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut cache = capital.into_iter().zip(profits).collect::<Vec<_>>();
        cache.sort_by_key(|&(c, _)| c);
        let mut index = 0;
        let mut max_heap = Vec::with_capacity(cache.len());
        let mut k = k;
        while k > 0 {
            for (i, &item) in cache.iter().enumerate().skip(index) {
                if item.0 <= w {
                    let pos = max_heap.binary_search(&item.1).unwrap_or_else(|e| e);
                    max_heap.insert(pos, item.1);
                    index = i + 1;
                } else {
                    break;
                }
            }
            if max_heap.is_empty() {
                break;
            }
            w += max_heap.pop().unwrap();
            k -= 1;
        }
        w
    }
}

#[test]
fn test_find_maximized_capital() {
    assert_eq!(Solution::find_maximized_capital(1, 2, vec![1, 2, 3], vec![1, 1, 2]), 5);
    assert_eq!(Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]), 4);
    assert_eq!(Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]), 6);
}
