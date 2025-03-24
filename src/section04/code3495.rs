#![allow(dead_code)]

// 3495. Minimum Operations to Make Array Elements Zero
// https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero/
// https://leetcode.cn/problems/minimum-operations-to-make-array-elements-zero/
//
// Hard
//
// You are given a 2D array queries, where queries[i] is of the form [l, r]. Each queries[i] defines
// an array of integers nums consisting of elements ranging from l to r, both inclusive.
//
// In one operation, you can:
//
//     Select two integers a and b from the array.
//     Replace them with floor(a / 4) and floor(b / 4).
//
// Your task is to determine the minimum number of operations required to reduce all elements
// of the array to zero for each query. Return the sum of the results for all queries.
//
// Example 1:
//
// Input: queries = [[1,2],[2,4]]
//
// Output: 3
//
// Explanation:
//
// For queries[0]:
//
//     The initial array is nums = [1, 2].
//     In the first operation, select nums[0] and nums[1]. The array becomes [0, 0].
//     The minimum number of operations required is 1.
//
// For queries[1]:
//
//     The initial array is nums = [2, 3, 4].
//     In the first operation, select nums[0] and nums[2]. The array becomes [0, 3, 1].
//     In the second operation, select nums[1] and nums[2]. The array becomes [0, 0, 0].
//     The minimum number of operations required is 2.
//
// The output is 1 + 2 = 3.
//
// Example 2:
//
// Input: queries = [[2,6]]
//
// Output: 4
//
// Explanation:
//
// For queries[0]:
//
//     The initial array is nums = [2, 3, 4, 5, 6].
//     In the first operation, select nums[0] and nums[3]. The array becomes [0, 3, 4, 1, 6].
//     In the second operation, select nums[2] and nums[4]. The array becomes [0, 3, 1, 1, 1].
//     In the third operation, select nums[1] and nums[2]. The array becomes [0, 0, 0, 1, 1].
//     In the fourth operation, select nums[3] and nums[4]. The array becomes [0, 0, 0, 0, 0].
//     The minimum number of operations required is 4.
//
// The output is 4.
//
// Constraints:
//
//     1 <= queries.length <= 10^5
//     queries[i].length == 2
//     queries[i] == [l, r]
//     1 <= l < r <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ans: i64 = 0;
        for query in queries {
            let start = query[0] as i64;
            let end = query[1] as i64;
            let mut ops: i64 = 0;
            let mut prev: i64 = 1;
            for d in 1..21 {
                let cur = prev * 4;
                let l = start.max(prev);
                let r = end.min(cur - 1);
                if r >= l {
                    ops += (r - l + 1) * d;
                }
                prev = cur;
            }
            ans += (ops + 1) / 2;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations(vec![vec![1, 2], vec![2, 4]]), 3);
    assert_eq!(Solution::min_operations(vec![vec![2, 6]]), 4);
}
