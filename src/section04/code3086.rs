#![allow(dead_code)]

// 3086. Minimum Moves to Pick K Ones
// https://leetcode.com/problems/minimum-moves-to-pick-k-ones/
// https://leetcode.cn/problems/minimum-moves-to-pick-k-ones/
//
// Hard
//
// You are given a binary array nums of length n, a positive integer k and a non-negative integer maxChanges.
//
// Alice plays a game, where the goal is for Alice to pick up k ones from nums using the minimum number of moves.
// When the game starts, Alice picks up any index aliceIndex in the range [0, n - 1] and stands there.
// If nums[aliceIndex] == 1 , Alice picks up the one and nums[aliceIndex] becomes 0(this does not count as a move).
// After this, Alice can make any number of moves (including zero) where in each move Alice must perform exactly one of the following actions:
//
// - Select any index j != aliceIndex such that nums[j] == 0 and set nums[j] = 1. This action can be performed at most maxChanges times.
// - Select any two adjacent indices x and y (|x - y| == 1) such that nums[x] == 1, nums[y] == 0,
//   then swap their values (set nums[y] = 1 and nums[x] = 0). If y == aliceIndex, Alice picks up the one after this move and nums[y] becomes 0.
//
// Return the minimum number of moves required by Alice to pick exactly k ones.
//
// Example 1:
//
// Input: nums = [1,1,0,0,0,1,1,0,0,1], k = 3, maxChanges = 1
//
// Output: 3
//
// Explanation: Alice can pick up 3 ones in 3 moves, if Alice performs the following actions in each move when standing at aliceIndex == 1:
//
// - At the start of the game Alice picks up the one and nums[1] becomes 0. nums becomes [1,1,1,0,0,1,1,0,0,1].
// - Select j == 2 and perform an action of the first type. nums becomes [1,0,1,0,0,1,1,0,0,1]
// - Select x == 2 and y == 1, and perform an action of the second type. nums becomes [1,1,0,0,0,1,1,0,0,1]. As y == aliceIndex,
//   Alice picks up the one and nums becomes [1,0,0,0,0,1,1,0,0,1].
// - Select x == 0 and y == 1, and perform an action of the second type. nums becomes [0,1,0,0,0,1,1,0,0,1]. As y == aliceIndex,
//   Alice picks up the one and nums becomes [0,0,0,0,0,1,1,0,0,1].
//
// Note that it may be possible for Alice to pick up 3 ones using some other sequence of 3 moves.
//
// Example 2:
//
// Input: nums = [0,0,0,0], k = 2, maxChanges = 3
//
// Output: 4
//
// Explanation: Alice can pick up 2 ones in 4 moves, if Alice performs the following actions in each move when standing at aliceIndex == 0:
//
// - Select j == 1 and perform an action of the first type. nums becomes [0,1,0,0].
// - Select x == 1 and y == 0, and perform an action of the second type. nums becomes [1,0,0,0].
//   As y == aliceIndex, Alice picks up the one and nums becomes [0,0,0,0].
// - Select j == 1 again and perform an action of the first type. nums becomes [0,1,0,0].
// - Select x == 1 and y == 0 again, and perform an action of the second type. nums becomes [1,0,0,0].
//   As y == aliceIndex, Alice picks up the one and nums becomes [0,0,0,0].
//
// Constraints:
//
//     2 <= n <= 10^5
//     0 <= nums[i] <= 1
//     1 <= k <= 10^5
//     0 <= maxChanges <= 10^5
//     maxChanges + sum(nums) >= k
//

struct Solution;

impl Solution {
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let n = nums.len();
        let mut ones = vec![];
        for (i, &nums_i) in nums.iter().enumerate().take(n) {
            if nums_i == 1 {
                ones.push(i);
            }
        }
        if ones.is_empty() {
            return 2 * k as i64;
        }
        let mut duo = -1;
        let mut trio = -1;
        for i in 0..n - 1 {
            if nums[i] == 1 && nums[i + 1] == 1 {
                duo = i as i32;
            }
        }
        for i in 0..n - 2 {
            if nums[i] + nums[i + 1] + nums[i + 2] == 3 {
                trio = i as i32;
            }
        }
        let mut hi = 1;
        if duo != -1 {
            hi = 2;
        }
        if trio != -1 {
            hi = 3;
        }
        if max_changes + hi >= k {
            hi = hi.min(k);
            let k = k - hi;
            return (hi - 1 + 2 * k) as _;
        }
        let m = ones.len();
        let mut p = vec![0; m + 1];
        for i in 0..m {
            p[i + 1] = p[i] + ones[i] as i64;
        }
        let k = k - max_changes;
        let precost = 2 * max_changes as i64;
        let mut ans = i64::MAX;
        for l in 0..m as i32 - k + 1 {
            let r = l + k - 1;
            let im = (r + l) / 2;
            let mid = ones[im as usize] as i64;
            let lsum = p[im as usize + 1] - p[l as usize];
            let rsum = p[r as usize + 1] - p[im as usize];
            let left = mid * (im - l + 1) as i64 - lsum;
            let right = rsum - mid * (r - im + 1) as i64;
            ans = ans.min(precost + left + right);
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1];
    let k = 3;
    let max_changes = 1;
    let res = 3;
    assert_eq!(Solution::minimum_moves(nums, k, max_changes), res);

    let nums = vec![0, 0, 0, 0];
    let k = 2;
    let max_changes = 3;
    let res = 4;
    assert_eq!(Solution::minimum_moves(nums, k, max_changes), res);
}
