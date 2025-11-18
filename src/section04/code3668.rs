#![allow(dead_code)]

// 3668. Restore Finishing Order
// https://leetcode.com/problems/restore-finishing-order/
// https://leetcode.cn/problems/restore-finishing-order/
//
// Easy
//
// You are given an integer array order of length n and an integer array friends.
//
// order contains every integer from 1 to n exactly once, representing the IDs of the participants of a race in their finishing order.
// friends contains the IDs of your friends in the race sorted in strictly increasing order. Each ID in friends is guaranteed to appear in the order array.
// Return an array containing your friends' IDs in their finishing order.
//
// Example 1:
//
// Input: order = [3,1,2,5,4], friends = [1,3,4]
//
// Output: [3,1,4]
//
// Explanation:
//
// The finishing order is [3, 1, 2, 5, 4]. Therefore, the finishing order of your friends is [3, 1, 4].
//
// Example 2:
//
// Input: order = [1,4,5,3,2], friends = [2,5]
//
// Output: [5,2]
//
// Explanation:
//
// The finishing order is [1, 4, 5, 3, 2]. Therefore, the finishing order of your friends is [5, 2].
//
// Constraints:
//
// 1 <= n == order.length <= 100
// order contains every integer from 1 to n exactly once
// 1 <= friends.length <= min(8, n)
// 1 <= friends[i] <= n
// friends is strictly increasing
//

struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        order.into_iter().filter(|n| friends.binary_search(n).is_ok()).collect()
    }
}

#[test]
fn test() {
    let order = vec![3, 1, 2, 5, 4];
    let friends = vec![1, 3, 4];
    let result = vec![3, 1, 4];
    assert_eq!(Solution::recover_order(order, friends), result);

    let order = vec![1, 4, 5, 3, 2];
    let friends = vec![2, 5];
    let result = vec![5, 2];
    assert_eq!(Solution::recover_order(order, friends), result);
}
