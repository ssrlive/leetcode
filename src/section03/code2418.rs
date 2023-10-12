#![allow(dead_code)]

// 2418. Sort the People
// https://leetcode.com/problems/sort-the-people/
// https://leetcode.cn/problems/sort-the-people/
//
// You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.
//
// For each index i, names[i] and heights[i] denote the name and height of the ith person.
//
// Return names sorted in descending order by the people's heights.
//
// Example 1:
//
// Input: names = ["Mary","John","Emma"], heights = [180,165,170]
// Output: ["Mary","Emma","John"]
// Explanation: Mary is the tallest, followed by Emma and John.
//
// Example 2:
//
// Input: names = ["Alice","Bob","Bob"], heights = [155,185,150]
// Output: ["Bob","Alice","Bob"]
// Explanation: The first Bob is the tallest, followed by Alice and the second Bob.
//
// Constraints:
//
// - n == names.length == heights.length
// - 1 <= n <= 10^3
// - 1 <= names[i].length <= 20
// - 1 <= heights[i] <= 10^5
// - names[i] consists of lower and upper case English letters.
// - All the values of heights are distinct.
//

struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut v = names.into_iter().zip(heights).collect::<Vec<_>>();
        v.sort_unstable_by_key(|(_, height)| -height);
        v.into_iter().map(|(x, _)| x).collect()
    }
}

#[test]
fn test() {
    let names = vec!["Mary", "John", "Emma"];
    let names = names.into_iter().map(|x| x.to_string()).collect();
    let heights = vec![180, 165, 170];
    let output = vec!["Mary", "Emma", "John"];
    assert_eq!(Solution::sort_people(names, heights), output);

    let names = vec!["Alice", "Bob", "Bob"];
    let names = names.into_iter().map(|x| x.to_string()).collect();
    let heights = vec![155, 185, 150];
    let output = vec!["Bob", "Alice", "Bob"];
    assert_eq!(Solution::sort_people(names, heights), output);
}
