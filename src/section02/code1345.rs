#![allow(dead_code)]

// 1345. Jump Game IV
// https://leetcode.com/problems/jump-game-iv/
// https://leetcode.cn/problems/jump-game-iv/
//
// Hard
//
// Given an array of integers arr, you are initially positioned at the first index of the array.
//
// In one step you can jump from index i to index:
//
//     i + 1 where: i + 1 < arr.length.
//     i - 1 where: i - 1 >= 0.
//     j where: arr[i] == arr[j] and i != j.
//
// Return the minimum number of steps to reach the last index of the array.
//
// Notice that you can not jump outside of the array at any time.
//
// Example 1:
//
// Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
// Output: 3
// Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
//
// Example 2:
//
// Input: arr = [7]
// Output: 0
// Explanation: Start index is the last index. You do not need to jump.
//
// Example 3:
//
// Input: arr = [7,6,9,6,9,6,9,7]
// Output: 1
// Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
//
// Constraints:
//
// -    1 <= arr.length <= 5 * 10^4
// -    -10^8 <= arr[i] <= 10^8
//

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};

        fn build_graph(nums: &[i32]) -> HashMap<i32, Vec<usize>> {
            let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
            for (idx, &num) in nums.iter().enumerate() {
                graph.entry(num).or_default().push(idx);
            }
            graph
        }

        let len_n: usize = arr.len();
        if len_n == 1 {
            return 0;
        }
        let mut graph = build_graph(&arr);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(0);
        let mut steps: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if cur == len_n - 1 {
                        return steps;
                    }
                    if let Some(nxts) = graph.get_mut(&arr[cur]) {
                        if cur >= 1 {
                            nxts.push(cur - 1);
                        }
                        if cur + 1 < len_n {
                            nxts.push(cur + 1);
                        }
                        while let Some(nxt) = nxts.pop() {
                            if seen.insert(nxt) {
                                queue.push_back(nxt);
                            }
                        }
                    };
                }
            }
            steps += 1;
        }
        steps
    }
}

#[test]
fn test() {
    let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
    assert_eq!(Solution::min_jumps(arr), 3);

    let arr = vec![7];
    assert_eq!(Solution::min_jumps(arr), 0);

    let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];
    assert_eq!(Solution::min_jumps(arr), 1);
}
