#![allow(dead_code)]

/*

// 1944. Number of Visible People in a Queue
// https://leetcode.com/problems/number-of-visible-people-in-a-queue/
// https://leetcode.cn/problems/number-of-visible-people-in-a-queue/
//
// Hard
//
// There are n people standing in a queue, and they numbered from 0 to n - 1 in left to right order. You are given an array heights of distinct integers where heights[i] represents the height of the ith person.

A person can see another person to their right in the queue if everybody in between is shorter than both of them. More formally, the ith person can see the jth person if i < j and min(heights[i], heights[j]) > max(heights[i+1], heights[i+2], ..., heights[j-1]).

Return an array answer of length n where answer[i] is the number of people the ith person can see to their right in the queue.

Example 1:

Input: heights = [10,6,8,5,11,9]
Output: [3,1,2,1,1,0]
Explanation:
Person 0 can see person 1, 2, and 4.
Person 1 can see person 2.
Person 2 can see person 3 and 4.
Person 3 can see person 4.
Person 4 can see person 5.
Person 5 can see no one since nobody is to the right of them.

Example 2:

Input: heights = [5,1,2,3,10]
Output: [4,1,1,1,0]

Constraints:

    n == heights.length
    1 <= n <= 10^5
    1 <= heights[i] <= 10^5
    All the values of heights are unique.
*/

struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut res = Vec::new();

        for idx in (0..heights.len()).rev() {
            let mut c = 0;
            while !stack.is_empty() && *stack.last().unwrap() < heights[idx] {
                stack.pop();
                c += 1;
            }
            if !stack.is_empty() {
                c += 1;
            }
            res.push(c);
            stack.push(heights[idx]);
        }

        res.into_iter().rev().collect()
    }
}

#[test]
fn test() {
    let heights = vec![10, 6, 8, 5, 11, 9];
    let res = vec![3, 1, 2, 1, 1, 0];
    assert_eq!(Solution::can_see_persons_count(heights), res);

    let heights = vec![5, 1, 2, 3, 10];
    let res = vec![4, 1, 1, 1, 0];
    assert_eq!(Solution::can_see_persons_count(heights), res);
}
