#![allow(dead_code)]

// 3161. Block Placement Queries
// https://leetcode.com/problems/block-placement-queries/
// https://leetcode.cn/problems/block-placement-queries/
//
// Hard
//
// There exists an infinite number line, with its origin at 0 and extending towards the positive x-axis.
//
// You are given a 2D array queries, which contains two types of queries:
//
// For a query of type 1, queries[i] = [1, x]. Build an obstacle at distance x from the origin. It is guaranteed that there is no obstacle at distance x when the query is asked.
// For a query of type 2, queries[i] = [2, x, sz]. Check if it is possible to place a block of size sz anywhere in the range [0, x] on the line, such that the block entirely lies in the range [0, x]. A block cannot be placed if it intersects with any obstacle, but it may touch it. Note that you do not actually place the block. Queries are separate.
// Return a boolean array results, where results[i] is true if you can place the block specified in the ith query of type 2, and false otherwise.
//
// Example 1:
//
// Input: queries = [[1,2],[2,3,3],[2,3,1],[2,2,2]]
//
// Output: [false,true,true]
//
// Explanation:
//
// For query 0, place an obstacle at x = 2. A block of size at most 2 can be placed before x = 3.
//
// Example 2:
//
// Input: queries = [[1,7],[2,7,6],[1,2],[2,7,5],[2,7,6]]
//
// Output: [true,true,false]
//
// Explanation:
//
// Place an obstacle at x = 7 for query 0. A block of size at most 7 can be placed before x = 7.
// Place an obstacle at x = 2 for query 2. Now, a block of size at most 5 can be placed before x = 7, and a block of size at most 2 before x = 2.
//
// Constraints:
//
// 1 <= queries.length <= 15 * 10^4
// 2 <= queries[i].length <= 3
// 1 <= queries[i][0] <= 2
// 1 <= x, sz <= min(5 * 10^4, 3 * queries.length)
// The input is generated such that for queries of type 1, no obstacle exists at distance x when the query is asked.
// The input is generated such that there is at least one query of type 2.
//

struct Solution;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut segment_tree = vec![0; std::cmp::min(50000, 3 * queries.len()) + 1];
        let mut ans: Vec<bool> = Vec::new();
        let mut blocks = std::collections::BTreeSet::new();

        for query in &queries {
            if query[0] == 1 {
                blocks.insert(query[1]);
            }
        }

        let mut blocks: Vec<i32> = blocks.into_iter().collect();
        blocks.insert(0, 0);

        Self::update_blocks(&mut blocks, &mut segment_tree);

        for query in queries.iter().rev() {
            let pos = match blocks.binary_search(&query[1]) {
                Ok(p) => p,
                Err(p) => p,
            };
            if query[0] == 1 {
                if pos + 1 < blocks.len() {
                    Self::update_segment_tree(&mut segment_tree, blocks[pos + 1] as usize, blocks[pos + 1] - blocks[pos - 1]);
                }
                blocks.remove(pos);
            } else {
                let can_place =
                    query[1] - blocks[pos - 1] >= query[2] || Self::query_segment_tree(&segment_tree, query[1] as isize) >= query[2];
                ans.push(can_place);
            }
        }

        ans.reverse();
        ans
    }

    fn update_segment_tree(segment_tree: &mut [i32], mut pos: usize, value: i32) {
        while pos < segment_tree.len() {
            segment_tree[pos] = std::cmp::max(segment_tree[pos], value);
            pos |= pos + 1;
        }
    }

    fn query_segment_tree(segment_tree: &[i32], mut pos: isize) -> i32 {
        let mut max_value = 0;
        while pos >= 0 {
            max_value = std::cmp::max(max_value, segment_tree[pos as usize]);
            pos = (pos & (pos + 1)) - 1;
        }
        max_value
    }

    fn update_blocks(blocks: &mut [i32], segment_tree: &mut [i32]) {
        for i in 1..blocks.len() {
            Self::update_segment_tree(segment_tree, blocks[i] as usize, blocks[i] - blocks[i - 1]);
        }
    }
}

#[test]
fn test() {
    let queries = vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]];
    let res = vec![false, true, true];
    assert_eq!(Solution::get_results(queries), res);

    let queries = vec![vec![1, 7], vec![2, 7, 6], vec![1, 2], vec![2, 7, 5], vec![2, 7, 6]];
    let res = vec![true, true, false];
    assert_eq!(Solution::get_results(queries), res);
}
