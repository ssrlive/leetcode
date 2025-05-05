#![allow(dead_code)]

// 3538. Merge Operations for Minimum Travel Time
// https://leetcode.com/problems/merge-operations-for-minimum-travel-time/
// https://leetcode.cn/problems/merge-operations-for-minimum-travel-time/
//
// Hard
//
// You are given a straight road of length l km, an integer n, an integer k, and two integer arrays, position and time, each of length n.
//
// The array position lists the positions (in km) of signs in strictly increasing order (with position[0] = 0 and position[n - 1] = l).
//
// Each time[i] represents the time (in minutes) required to travel 1 km between position[i] and position[i + 1].
//
// You must perform exactly k merge operations. In one merge, you can choose any two adjacent signs at indices i and i + 1 (with i > 0 and i + 1 < n) and:
//
//     Update the sign at index i + 1 so that its time becomes time[i] + time[i + 1].
//     Remove the sign at index i.
//
// Return the minimum total travel time (in minutes) to travel from 0 to l after exactly k merges.
//
// Example 1:
//
// Input: l = 10, n = 4, k = 1, position = [0,3,8,10], time = [5,8,3,6]
//
// Output: 62
//
// Explanation:
//
//     Merge the signs at indices 1 and 2. Remove the sign at index 1, and change the time at index 2 to 8 + 3 = 11.
//     After the merge:
//         position array: [0, 8, 10]
//         time array: [5, 11, 6]
//     Segment	Distance (km)	Time per km (min)	Segment Travel Time (min)
//     0 → 8	8	5	8 × 5 = 40
//     8 → 10	2	11	2 × 11 = 22
//     Total Travel Time: 40 + 22 = 62, which is the minimum possible time after exactly 1 merge.
//
// Example 2:
//
// Input: l = 5, n = 5, k = 1, position = [0,1,2,3,5], time = [8,3,9,3,3]
//
// Output: 34
//
// Explanation:
//
//     Merge the signs at indices 1 and 2. Remove the sign at index 1, and change the time at index 2 to 3 + 9 = 12.
//     After the merge:
//         position array: [0, 2, 3, 5]
//         time array: [8, 12, 3, 3]
//     Segment	Distance (km)	Time per km (min)	Segment Travel Time (min)
//     0 → 2	2	8	2 × 8 = 16
//     2 → 3	1	12	1 × 12 = 12
//     3 → 5	2	3	2 × 3 = 6
//     Total Travel Time: 16 + 12 + 6 = 34, which is the minimum possible time after exactly 1 merge.
//
// Constraints:
//
//     1 <= l <= 10^5
//     2 <= n <= min(l + 1, 50)
//     0 <= k <= min(n - 2, 10)
//     position.length == n
//     position[0] = 0 and position[n - 1] = l
//     position is sorted in strictly increasing order.
//     time.length == n
//     1 <= time[i] <= 100​
//     1 <= sum(time) <= 100​​​​​​
//

struct Solution;

impl Solution {
    pub fn min_travel_time(_l: i32, n: i32, k: i32, position: Vec<i32>, time: Vec<i32>) -> i32 {
        #[allow(clippy::too_many_arguments)]
        fn recursion(
            pos: usize,
            curr_k: i32,
            curr_time: i32,
            n: usize,
            position: &[i32],
            time: &[i32],
            dp: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if pos == n - 1 {
                return if curr_k > 0 { i32::MAX } else { 0 };
            }

            if dp[pos][curr_k as usize][curr_time as usize] != -1 {
                return dp[pos][curr_k as usize][curr_time as usize];
            }

            let mut ans = i32::MAX;

            let res = recursion(pos + 1, curr_k, time[pos + 1], n, position, time, dp);
            if res != i32::MAX {
                ans = ans.min((position[pos + 1] - position[pos]) * curr_time + res);
            }

            if curr_k > 0 {
                let mut time_sum = time[pos + 1];
                let mut operations = 0;
                for next_idx in pos + 2..=std::cmp::min(n - 1, pos + curr_k as usize + 1) {
                    time_sum += time[next_idx];
                    operations += 1;

                    let res = recursion(next_idx, curr_k - operations, time_sum, n, position, time, dp);
                    if res != i32::MAX {
                        ans = ans.min((position[next_idx] - position[pos]) * curr_time + res);
                    }
                }
            }

            dp[pos][curr_k as usize][curr_time as usize] = ans;
            ans
        }

        let sum: i32 = time.iter().sum();
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![vec![-1; (sum + 1) as usize]; k + 1]; n];
        recursion(0, k as i32, time[0], n, &position, &time, &mut dp)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_travel_time(10, 4, 1, vec![0, 3, 8, 10], vec![5, 8, 3, 6]), 62);
    assert_eq!(Solution::min_travel_time(5, 5, 1, vec![0, 1, 2, 3, 5], vec![8, 3, 9, 3, 3]), 34);
}
