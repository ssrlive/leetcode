#![allow(dead_code)]

// 2558. Take Gifts From the Richest Pile
// https://leetcode.com/problems/take-gifts-from-the-richest-pile/
// https://leetcode.cn/problems/take-gifts-from-the-richest-pile/
//
// Easy
//
// You are given an integer array gifts denoting the number of gifts in various piles. Every second, you do the following:
//
//     Choose the pile with the maximum number of gifts.
//     If there is more than one pile with the maximum number of gifts, choose any.
//     Leave behind the floor of the square root of the number of gifts in the pile. Take the rest of the gifts.
//
// Return the number of gifts remaining after k seconds.
//
// Example 1:
//
// Input: gifts = [25,64,9,4,100], k = 4
// Output: 29
// Explanation:
// The gifts are taken in the following way:
// - In the first second, the last pile is chosen and 10 gifts are left behind.
// - Then the second pile is chosen and 8 gifts are left behind.
// - After that the first pile is chosen and 5 gifts are left behind.
// - Finally, the last pile is chosen again and 3 gifts are left behind.
// The final remaining gifts are [5,8,9,4,3], so the total number of gifts remaining is 29.
//
// Example 2:
//
// Input: gifts = [1,1,1,1], k = 4
// Output: 4
// Explanation:
// In this case, regardless which pile you choose, you have to leave behind 1 gift in each pile.
// That is, you can't take any pile with you.
// So, the total gifts remaining are 4.
//
// Constraints:
//
// -    1 <= gifts.length <= 10^3
// -    1 <= gifts[i] <= 10^9
// -    1 <= k <= 10^3
//

struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut k = k;
        let mut ans = 0;
        let mut pq = std::collections::BinaryHeap::new();
        for i in gifts {
            pq.push(i);
        }
        while k > 0 && !pq.is_empty() {
            let t = pq.pop().unwrap();
            pq.push((t as f64).sqrt() as i32);
            k -= 1;
        }
        while !pq.is_empty() {
            ans += pq.pop().unwrap() as i64;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
    assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
}
