#![allow(dead_code)]

// 2555. Maximize Win From Two Segments
// https://leetcode.com/problems/maximize-win-from-two-segments/
// https://leetcode.cn/problems/maximize-win-from-two-segments/
//
// Medium
//
// There are some prizes on the X-axis. You are given an integer array prizePositions that is sorted
// in non-decreasing order, where prizePositions[i] is the position of the ith prize.
// There could be different prizes at the same position on the line. You are also given an integer k.
//
// You are allowed to select two segments with integer endpoints. The length of each segment must be k.
// You will collect all prizes whose position falls within at least one of the two selected segments
// (including the endpoints of the segments). The two selected segments may intersect.
//
// -    For example if k = 2, you can choose segments [1, 3] and [2, 4], and you will win any prize i
//      that satisfies 1 <= prizePositions[i] <= 3 or 2 <= prizePositions[i] <= 4.
//
// Return the maximum number of prizes you can win if you choose the two segments optimally.
//
// Example 1:
//
// Input: prizePositions = [1,1,2,2,3,3,5], k = 2
// Output: 7
// Explanation: In this example, you can win all 7 prizes by selecting two segments [1, 3] and [3, 5].
//
// Example 2:
//
// Input: prizePositions = [1,2,3,4], k = 0
// Output: 2
// Explanation: For this example, one choice for the segments is [3, 3] and [4, 4], and you will be able to get 2 prizes.
//
// Constraints:
//
// -    1 <= prizePositions.length <= 10^5
// -    1 <= prizePositions[i] <= 10^9
// -    0 <= k <= 10^9
// -    prizePositions is sorted in non-decreasing order.
//

struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let mut data = Vec::<(i32, i32)>::new();
        for p in prize_positions {
            if !data.is_empty() && data[data.len() - 1].0 == p {
                let m = data.len();
                data[m - 1].1 += 1;
            } else {
                data.push((p, 1));
            }
        }

        let n = data.len();
        let mut right = vec![0; n];
        let mut j = n - 1;
        let (mut ret, mut sum) = (0, 0);

        for i in (0..n).rev() {
            sum += data[i].1;
            while data[j].0 - k > data[i].0 {
                sum -= data[j].1;
                j -= 1;
            }
            right[i] = sum;
            if i + 1 < n {
                right[i] = right[i].max(right[i + 1]);
            }

            let mut t = sum;
            if j + 1 < n {
                t += right[j + 1];
            }
            ret = ret.max(t);
        }

        ret
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 1, 2, 2, 3, 3, 5], 2, 7), (vec![1, 2, 3, 4], 0, 2)];
    for (prize_positions, k, expected) in cases {
        assert_eq!(Solution::maximize_win(prize_positions, k), expected);
    }
}
