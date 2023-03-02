#![allow(dead_code)]

/*

// 2058. Find the Minimum and Maximum Number of Nodes Between Critical Points
// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/
// https://leetcode.cn/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/
//
// Medium
//
// A critical point in a linked list is defined as either a local maxima or a local minima.

A node is a local maxima if the current node has a value strictly greater than the previous node and the next node.

A node is a local minima if the current node has a value strictly smaller than the previous node and the next node.

Note that a node can only be a local maxima/minima if there exists both a previous node and a next node.

Given a linked list head, return an array of length 2 containing [minDistance, maxDistance] where minDistance is the minimum distance between any two distinct critical points and maxDistance is the maximum distance between any two distinct critical points. If there are fewer than two critical points, return [-1, -1].

Example 1:

Input: head = [3,1]
Output: [-1,-1]
Explanation: There are no critical points in [3,1].

Example 2:

Input: head = [5,3,1,2,5,1,2]
Output: [1,3]
Explanation: There are three critical points:
- [5,3,1,2,5,1,2]: The third node is a local minima because 1 is less than 3 and 2.
- [5,3,1,2,5,1,2]: The fifth node is a local maxima because 5 is greater than 2 and 1.
- [5,3,1,2,5,1,2]: The sixth node is a local minima because 1 is less than 5 and 2.
The minimum distance is between the fifth and the sixth node. minDistance = 6 - 5 = 1.
The maximum distance is between the third and the sixth node. maxDistance = 6 - 3 = 3.

Example 3:

Input: head = [1,3,2,2,3,2,2,2,7]
Output: [3,3]
Explanation: There are two critical points:
- [1,3,2,2,3,2,2,2,7]: The second node is a local maxima because 3 is greater than 1 and 2.
- [1,3,2,2,3,2,2,2,7]: The fifth node is a local maxima because 3 is greater than 2 and 2.
Both the minimum and maximum distances are between the second and the fifth node.
Thus, minDistance and maxDistance is 5 - 2 = 3.
Note that the last node is not considered a local maxima because it does not have a next node.

Constraints:

    The number of nodes in the list is in the range [2, 10^5].
    1 <= Node.val <= 10^5
*/

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        if let Some(node) = head {
            let mut now = node;
            let mut temp = vec![now.val];
            while let Some(next) = now.next {
                temp.push(next.val);
                now = next;
            }

            let n = temp.len();
            let mut memo = vec![];
            for i in 1..n - 1 {
                if (temp[i] < temp[i + 1] && temp[i] < temp[i - 1]) || (temp[i] > temp[i + 1] && temp[i] > temp[i - 1])
                {
                    memo.push(i);
                }
            }

            if memo.len() <= 1 {
                vec![-1, -1]
            } else {
                let m = memo.len();
                let max = memo[m - 1] - memo[0];
                let mut min = memo[1] - memo[0];
                for i in 0..m - 1 {
                    min = min.min(memo[i + 1] - memo[i]);
                }
                vec![min as i32, max as i32]
            }
        } else {
            vec![-1, -1]
        }
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[5, 3, 1, 2, 5, 1, 2]);
    let result = Solution::nodes_between_critical_points(head);
    assert_eq!(result, vec![1, 3]);

    let head = ListNode::from_vec(&[1, 3, 2, 2, 3, 2, 2, 2, 7]);
    let result = Solution::nodes_between_critical_points(head);
    assert_eq!(result, vec![3, 3]);

    let head = ListNode::from_vec(&[3, 1]);
    let result = Solution::nodes_between_critical_points(head);
    assert_eq!(result, vec![-1, -1]);
}
