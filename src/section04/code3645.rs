#![allow(dead_code)]

// 3645. Maximum Total from Optimal Activation Order
// https://leetcode.com/problems/maximum-total-from-optimal-activation-order/
// https://leetcode.cn/problems/maximum-total-from-optimal-activation-order/
//
// Medium
//
// You are given two integer arrays value and limit, both of length n.
//
// Initially, all elements are inactive. You may activate them in any order.
//
// To activate an inactive element at index i, the number of currently active elements must be strictly less than limit[i].
// When you activate the element at index i, it adds value[i] to the total activation value (i.e., the sum of value[i] for all elements that have undergone activation operations).
// After each activation, if the number of currently active elements becomes x, then all elements j with limit[j] <= x become permanently inactive, even if they are already active.
// Return the maximum total you can obtain by choosing the activation order optimally.
//
// Example 1:
//
// Input: value = [3,5,8], limit = [2,1,3]
//
// Output: 16
//
// Explanation:
//
// One optimal activation order is:
//
// Step	Activated i	value[i]	Active Before i	Active After i	Becomes Inactive j	Inactive Elements	Total
// 1	1	5	0	1	j = 1 as limit[1] = 1	[1]	5
// 2	0	3	0	1	-	[1]	8
// 3	2	8	1	2	j = 0 as limit[0] = 2	[0, 1]	16
// Thus, the maximum possible total is 16.
//
// Example 2:
//
// Input: value = [4,2,6], limit = [1,1,1]
//
// Output: 6
//
// Explanation:
//
// One optimal activation order is:
//
// Step	Activated i	value[i]	Active Before i	Active After i	Becomes Inactive j	Inactive Elements	Total
// 1	2	6	0	1	j = 0, 1, 2 as limit[j] = 1	[0, 1, 2]	6
// Thus, the maximum possible total is 6.
//
// Example 3:
//
// Input: value = [4,1,5,2], limit = [3,3,2,3]
//
// Output: 12
//
// Explanation:
//
// One optimal activation order is:​​​​​​​​​​​​​​
//
// Step	Activated i	value[i]	Active Before i	Active After i	Becomes Inactive j	Inactive Elements	Total
// 1	2	5	0	1	-	[ ]	5
// 2	0	4	1	2	j = 2 as limit[2] = 2	[2]	9
// 3	1	1	1	2	-	[2]	10
// 4	3	2	2	3	j = 0, 1, 3 as limit[j] = 3	[0, 1, 2, 3]	12
// Thus, the maximum possible total is 12.
//
// Constraints:
//
// 1 <= n == value.length == limit.length <= 105
// 1 <= value[i] <= 105​​​​​​​
// 1 <= limit[i] <= n
//

struct Solution;

impl Solution {
    pub fn max_total(value: Vec<i32>, limit: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let n = value.len();
        let mut buckets: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..n {
            buckets.entry(limit[i]).or_default().push(value[i]);
        }
        let mut ans: i64 = 0;
        for (l, arr) in buckets {
            let mut arr = arr;
            arr.sort_unstable_by(|a, b| b.cmp(a));
            let take = l.min(arr.len() as i32) as usize;
            for &arr_i in arr.iter().take(take) {
                ans += arr_i as i64;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let value = vec![3, 5, 8];
    let limit = vec![2, 1, 3];
    assert_eq!(Solution::max_total(value, limit), 16);

    let value = vec![4, 2, 6];
    let limit = vec![1, 1, 1];
    assert_eq!(Solution::max_total(value, limit), 6);

    let value = vec![4, 1, 5, 2];
    let limit = vec![3, 3, 2, 3];
    assert_eq!(Solution::max_total(value, limit), 12);
}
