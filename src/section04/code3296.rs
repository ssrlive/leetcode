#![allow(dead_code)]

// 3296. Minimum Number of Seconds to Make Mountain Height Zero
// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/
// https://leetcode.cn/problems/minimum-number-of-seconds-to-make-mountain-height-zero/
//
// Medium
//
// You are given an integer mountainHeight denoting the height of a mountain.
//
// You are also given an integer array workerTimes representing the work time of workers in seconds.
//
// The workers work simultaneously to reduce the height of the mountain. For worker i:
//
// To decrease the mountain's height by x, it takes workerTimes[i] + workerTimes[i] * 2 + ... + workerTimes[i] * x seconds. For example:
// To reduce the height of the mountain by 1, it takes workerTimes[i] seconds.
// To reduce the height of the mountain by 2, it takes workerTimes[i] + workerTimes[i] * 2 seconds, and so on.
// Return an integer representing the minimum number of seconds required for the workers to make the height of the mountain 0.
//
// Example 1:
//
// Input: mountainHeight = 4, workerTimes = [2,1,1]
//
// Output: 3
//
// Explanation:
//
// One way the height of the mountain can be reduced to 0 is:
//
// Worker 0 reduces the height by 1, taking workerTimes[0] = 2 seconds.
// Worker 1 reduces the height by 2, taking workerTimes[1] + workerTimes[1] * 2 = 3 seconds.
// Worker 2 reduces the height by 1, taking workerTimes[2] = 1 second.
// Since they work simultaneously, the minimum time needed is max(2, 3, 1) = 3 seconds.
//
// Example 2:
//
// Input: mountainHeight = 10, workerTimes = [3,2,2,4]
//
// Output: 12
//
// Explanation:
//
// Worker 0 reduces the height by 2, taking workerTimes[0] + workerTimes[0] * 2 = 9 seconds.
// Worker 1 reduces the height by 3, taking workerTimes[1] + workerTimes[1] * 2 + workerTimes[1] * 3 = 12 seconds.
// Worker 2 reduces the height by 3, taking workerTimes[2] + workerTimes[2] * 2 + workerTimes[2] * 3 = 12 seconds.
// Worker 3 reduces the height by 2, taking workerTimes[3] + workerTimes[3] * 2 = 12 seconds.
// The number of seconds needed is max(9, 12, 12, 12) = 12 seconds.
//
// Example 3:
//
// Input: mountainHeight = 5, workerTimes = [1]
//
// Output: 15
//
// Explanation:
//
// There is only one worker in this example, so the answer is
// workerTimes[0] + workerTimes[0] * 2 + workerTimes[0] * 3 + workerTimes[0] * 4 + workerTimes[0] * 5 = 15.
//
// Constraints:
//
// 1 <= mountainHeight <= 10^5
// 1 <= workerTimes.length <= 10^4
// 1 <= workerTimes[i] <= 10^6
//

struct Solution;

impl Solution {
    fn helper(a: f64) -> i64 {
        ((f64::sqrt(4.0 * a + 1.0) - 1.0) / 2.0).floor() as i64
    }

    fn is_possible(h: i32, arr: &Vec<i32>, num_seconds: i64) -> bool {
        let mut res = 0;
        for &w in arr {
            res += Self::helper(2.0 * num_seconds as f64 / w as f64);
        }
        res >= h as i64
    }

    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut lo = 0;
        let mut hi = 9000000000000000;
        while lo < hi {
            let mi = (lo + hi) / 2;
            if Self::is_possible(mountain_height, &worker_times, mi) {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        lo
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_number_of_seconds(4, vec![2, 1, 1]), 3);
    assert_eq!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
    assert_eq!(Solution::min_number_of_seconds(5, vec![1]), 15);
}
