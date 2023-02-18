#![allow(dead_code)]

/*

// 1707. Maximum XOR With an Element From Array
Hard
860
28
Companies

You are given an array nums consisting of non-negative integers. You are also given a queries array, where queries[i] = [xi, mi].

The answer to the ith query is the maximum bitwise XOR value of xi and any element of nums that does not exceed mi. In other words, the answer is max(nums[j] XOR xi) for all j such that nums[j] <= mi. If all elements in nums are larger than mi, then the answer is -1.

Return an integer array answer where answer.length == queries.length and answer[i] is the answer to the ith query.

Example 1:

Input: nums = [0,1,2,3,4], queries = [[3,1],[1,3],[5,6]]
Output: [3,3,7]
Explanation:
1) 0 and 1 are the only two integers not greater than 1. 0 XOR 3 = 3 and 1 XOR 3 = 2. The larger of the two is 3.
2) 1 XOR 2 = 3.
3) 5 XOR 2 = 7.

Example 2:

Input: nums = [5,2,4,6,6,3], queries = [[12,4],[8,1],[6,3]]
Output: [15,-1,5]

Constraints:

    1 <= nums.length, queries.length <= 10^5
    queries[i].length == 2
    0 <= nums[j], xi, mi <= 10^9
*/

struct Solution;

impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut trie = Trie::new();
        let mut nums = nums;
        nums.sort();
        let mut queries = queries
            .into_iter()
            .enumerate()
            .map(|(i, q)| (q[1], q[0], i))
            .collect::<Vec<_>>();
        queries.sort();
        let mut v1 = vec![0; queries.len()];
        let mut st = 0;
        let en = nums.len();
        for (m, x, i) in queries {
            while st < en && nums[st] <= m {
                trie.insert(nums[st]);
                st += 1;
            }
            if st == 0 {
                v1[i] = -1;
            } else {
                v1[i] = trie.maxnum(x);
            }
        }
        v1
    }
}

use std::cell::RefCell;
use std::rc::Rc;

type OptNode = Option<Rc<RefCell<Node>>>;
struct Node {
    links: [OptNode; 2],
}

impl Node {
    fn new() -> Self {
        Self { links: [None, None] }
    }
}

struct Trie {
    root: OptNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(Node::new()))),
        }
    }

    fn insert(&mut self, num: i32) {
        let num = num as usize;
        let mut node = self.root.clone();
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            {
                let mut ref_node = node.as_ref().unwrap().borrow_mut();
                if ref_node.links[bit].is_none() {
                    ref_node.links[bit] = Some(Rc::new(RefCell::new(Node::new())));
                }
            }
            let v = node.as_ref().unwrap().borrow().links[bit].clone();
            node = v;
        }
    }

    fn maxnum(&self, num: i32) -> i32 {
        let mut maxi = 0;
        let mut node = self.root.clone();
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            let v = if node.as_ref().unwrap().borrow().links[(1 - bit) as usize].is_some() {
                maxi |= 1 << i;
                node.as_ref().unwrap().borrow().links[(1 - bit) as usize].clone()
            } else {
                node.as_ref().unwrap().borrow().links[bit as usize].clone()
            };
            node = v;
        }
        maxi
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![0, 1, 2, 3, 4],
            vec![vec![3, 1], vec![1, 3], vec![5, 6]],
            vec![3, 3, 7],
        ),
        (
            vec![5, 2, 4, 6, 6, 3],
            vec![vec![12, 4], vec![8, 1], vec![6, 3]],
            vec![15, -1, 5],
        ),
    ];
    for (nums, queries, expect) in cases {
        assert_eq!(Solution::maximize_xor(nums, queries), expect);
    }
}
