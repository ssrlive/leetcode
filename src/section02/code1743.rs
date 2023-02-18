#![allow(dead_code)]

/*

// 1743. Restore the Array From Adjacent Pairs
Medium
866
17
Companies

There is an integer array nums that consists of n unique elements, but you have forgotten it. However, you do remember every pair of adjacent elements in nums.

You are given a 2D integer array adjacentPairs of size n - 1 where each adjacentPairs[i] = [ui, vi] indicates that the elements ui and vi are adjacent in nums.

It is guaranteed that every adjacent pair of elements nums[i] and nums[i+1] will exist in adjacentPairs, either as [nums[i], nums[i+1]] or [nums[i+1], nums[i]]. The pairs can appear in any order.

Return the original array nums. If there are multiple solutions, return any of them.

Example 1:

Input: adjacentPairs = [[2,1],[3,4],[3,2]]
Output: [1,2,3,4]
Explanation: This array has all its adjacent pairs in adjacentPairs.
Notice that adjacentPairs[i] may not be in left-to-right order.

Example 2:

Input: adjacentPairs = [[4,-2],[1,4],[-3,1]]
Output: [-2,4,1,-3]
Explanation: There can be negative numbers.
Another solution is [-3,1,4,-2], which would also be accepted.

Example 3:

Input: adjacentPairs = [[100000,-100000]]
Output: [100000,-100000]

Constraints:

    nums.length == n
    adjacentPairs.length == n - 1
    adjacentPairs[i].length == 2
    2 <= n <= 10^5
    -105 <= nums[i], ui, vi <= 10^5
    There exists some nums that has adjacentPairs as its pairs.
*/

struct Solution;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for pair in &adjacent_pairs {
            map.entry(pair[1]).or_insert(vec![]).push(pair[0]);
            map.entry(pair[0]).or_insert(vec![]).push(pair[1]);
        }
        let start = map
            .iter()
            .find_map(|(k, v)| if v.len() == 1 { Some(*k) } else { None })
            .unwrap();
        let mut stack = vec![start];
        let mut seen = std::collections::HashSet::new();
        let mut result = vec![];
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            result.push(current);
            seen.insert(current);
            for k in map.get(&current).unwrap() {
                if !seen.contains(k) {
                    stack.push(*k);
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![2, 1], vec![3, 4], vec![3, 2]], vec![1, 2, 3, 4]),
        (vec![vec![4, -2], vec![1, 4], vec![-3, 1]], vec![-3, -2, 1, 4]),
        (vec![vec![100000, -100000]], vec![-100000, 100000]),
    ];
    for (adjacent_pairs, expected) in cases {
        let mut result = Solution::restore_array(adjacent_pairs);
        result.sort();
        assert_eq!(result, expected);
    }
}
