#![allow(dead_code)]

/*

// 2569. Handling Sum Queries After Update
// https://leetcode.com/problems/handling-sum-queries-after-update/
// https://leetcode.cn/problems/handling-sum-queries-after-update/
//
// Hard
//
// You are given two 0-indexed arrays nums1 and nums2 and a 2D array queries of queries. There are three types of queries:

    For a query of type 1, queries[i] = [1, l, r]. Flip the values from 0 to 1 and from 1 to 0 in nums1 from index l to index r. Both l and r are 0-indexed.
    For a query of type 2, queries[i] = [2, p, 0]. For every index 0 <= i < n, set nums2[i] = nums2[i] + nums1[i] * p.
    For a query of type 3, queries[i] = [3, 0, 0]. Find the sum of the elements in nums2.

Return an array containing all the answers to the third type queries.

Example 1:

Input: nums1 = [1,0,1], nums2 = [0,0,0], queries = [[1,1,1],[2,1,0],[3,0,0]]
Output: [3]
Explanation: After the first query nums1 becomes [1,1,1]. After the second query, nums2 becomes [1,1,1], so the answer to the third query is 3. Thus, [3] is returned.

Example 2:

Input: nums1 = [1], nums2 = [5], queries = [[2,0,0],[3,0,0]]
Output: [5]
Explanation: After the first query, nums2 remains [5], so the answer to the second query is 5. Thus, [5] is returned.

Constraints:

    1 <= nums1.length,nums2.length <= 10^5
    nums1.length = nums2.length
    1 <= queries.length <= 10^5
    queries[i].length = 3
    0 <= l <= r <= nums1.length - 1
    0 <= p <= 10^6
    0 <= nums1[i] <= 1
    0 <= nums2[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        fn build(arr: &Vec<i64>, n: usize, a: usize, b: usize, tree: &mut Vec<i64>) -> i64 {
            if a == b {
                tree[n] = arr[a];
                return tree[n];
            }
            tree[n] = build(arr, 2 * n, a, (a + b) / 2, tree) + build(arr, 2 * n + 1, (a + b) / 2 + 1, b, tree);
            tree[n]
        }

        fn update_tree(n: usize, a: i64, b: i64, i: i64, j: i64, tree: &mut Vec<i64>, lazy: &mut Vec<i64>) -> i64 {
            if b < i || a > j {
                return if lazy[n] == 1 { b - a + 1 - tree[n] } else { tree[n] };
            }
            if lazy[n] == 1 {
                tree[n] = b - a + 1 - tree[n];
                if a != b {
                    lazy[2 * n] = 1 - lazy[2 * n];
                    lazy[2 * n + 1] = 1 - lazy[2 * n + 1];
                }
                lazy[n] = 0;
            }
            if a >= i && b <= j {
                if a != b {
                    lazy[2 * n] = 1 - lazy[2 * n];
                    lazy[2 * n + 1] = 1 - lazy[2 * n + 1];
                }
                tree[n] = b - a + 1 - tree[n];
                return tree[n];
            }
            tree[n] = update_tree(2 * n, a, (a + b) / 2, i, j, tree, lazy) + update_tree(2 * n + 1, (a + b) / 2 + 1, b, i, j, tree, lazy);
            tree[n]
        }

        let nums1 = nums1.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let nums2 = nums2.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let f = |x: &Vec<i32>| x.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let queries = queries.iter().map(f).collect::<Vec<_>>();

        let mut tree = vec![0; 400000];
        let mut lazy = vec![0; 400000];
        let mut sum = nums2.iter().sum::<_>();
        let sz = nums1.len();
        build(&nums1, 1, 0, sz - 1, &mut tree);
        let mut res = vec![];
        for q in queries {
            if q[0] == 1 {
                update_tree(1, 0, sz as i64 - 1, q[1], q[2], &mut tree, &mut lazy);
            } else if q[0] == 2 {
                sum += tree[1] * q[1];
            } else {
                res.push(sum);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![vec![1, 1, 1], vec![2, 1, 0], vec![3, 0, 0]],
            vec![3],
        ),
        (vec![1], vec![5], vec![vec![2, 0, 0], vec![3, 0, 0]], vec![5]),
    ];
    for (nums1, nums2, queries, expect) in cases {
        assert_eq!(Solution::handle_query(nums1, nums2, queries), expect);
    }
}
