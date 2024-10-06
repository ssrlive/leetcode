#![allow(dead_code)]

// 3310. Remove Methods From Project
// https://leetcode.com/problems/remove-methods-from-project/
// https://leetcode.cn/problems/remove-methods-from-project/
//
// Medium
//
// You are maintaining a project that has n methods numbered from 0 to n - 1.
//
// You are given two integers n and k, and a 2D integer array invocations, where invocations[i] = [ai, bi] indicates that method ai invokes method bi.
//
// There is a known bug in method k. Method k, along with any method invoked by it, either directly or indirectly, are considered suspicious and we aim to remove them.
//
// A group of methods can only be removed if no method outside the group invokes any methods within it.
//
// Return an array containing all the remaining methods after removing all the suspicious methods. You may return the answer in any order. If it is not possible to remove all the suspicious methods, none should be removed.
//
// Example 1:
//
// Input: n = 4, k = 1, invocations = [[1,2],[0,1],[3,2]]
//
// Output: [0,1,2,3]
//
// Explanation:
//
// Method 2 and method 1 are suspicious, but they are directly invoked by methods 3 and 0, which are not suspicious. We return all elements without removing anything.
//
// Example 2:
//
// Input: n = 5, k = 0, invocations = [[1,2],[0,2],[0,1],[3,4]]
//
// Output: [3,4]
//
// Explanation:
//
// Methods 0, 1, and 2 are suspicious and they are not directly invoked by any other method. We can remove them.
//
// Example 3:
//
// Input: n = 3, k = 2, invocations = [[1,2],[0,1],[2,0]]
//
// Output: []
//
// Explanation:
//
// All methods are suspicious. We can remove them.
//
// Constraints:
//
//     1 <= n <= 10^5
//     0 <= k <= n - 1
//     0 <= invocations.length <= 2 * 10^5
//     invocations[i] == [ai, bi]
//     0 <= ai, bi <= n - 1
//     ai != bi
//     invocations[i] != invocations[j]
//

struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let m = invocations.len() as i32;
        let mut st = std::collections::HashSet::new();
        let mut adj = vec![vec![]; n as usize];
        for it in invocations.iter() {
            let u = it[0] as usize;
            let v = it[1] as usize;
            adj[u].push(v);
        }

        let mut q = std::collections::VecDeque::new();
        q.push_back(k);
        st.insert(k);
        let mut visited = vec![0; n as usize];
        visited[k as usize] = 1;
        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            for it in adj[node as usize].iter() {
                if visited[*it] == 0 {
                    st.insert(*it as i32);
                    q.push_back(*it as i32);
                    visited[*it] = 1;
                }
            }
        }

        let mut flag = false;

        for i in 0..m {
            let x = invocations[i as usize][1];
            let y = invocations[i as usize][0];
            if !st.contains(&y) && st.contains(&x) {
                flag = true;
            }
        }

        let mut ans = vec![];
        if flag {
            for i in 0..n {
                ans.push(i);
            }
        } else {
            for i in 0..n {
                if !st.contains(&i) {
                    ans.push(i);
                }
            }
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        Solution::remaining_methods(5, 0, vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]]),
        vec![3, 4]
    );
    assert_eq!(Solution::remaining_methods(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]]), vec![]);
}
