#![allow(dead_code)]

// 3187. Peaks in Array
// https://leetcode.com/problems/peaks-in-array/
// https://leetcode.cn/problems/peaks-in-array/
//
// Hard
//
// A peak in an array arr is an element that is greater than its previous and next element in arr.
//
// You are given an integer array nums and a 2D integer array queries.
//
// You have to process queries of two types:
//
//     queries[i] = [1, li, ri], determine the count of peak elements in the
//     subarray
//     nums[li..ri].
//     queries[i] = [2, indexi, vali], change nums[indexi] to vali.
//
// Return an array answer containing the results of the queries of the first type in order.
//
// Notes:
//
//     The first and the last element of an array or a subarray cannot be a peak.
//
// Example 1:
//
// Input: nums = [3,1,4,2,5], queries = [[2,3,4],[1,0,4]]
//
// Output: [0]
//
// Explanation:
//
// First query: We change nums[3] to 4 and nums becomes [3,1,4,4,5].
//
// Second query: The number of peaks in the [3,1,4,4,5] is 0.
//
// Example 2:
//
// Input: nums = [4,1,4,2,1,5], queries = [[2,2,4],[1,0,2],[1,0,4]]
//
// Output: [0,1]
//
// Explanation:
//
// First query: nums[2] should become 4, but it is already set to 4.
//
// Second query: The number of peaks in the [4,1,4] is 0.
//
// Third query: The second 4 is a peak in the [4,1,4,2,1].
//
// Constraints:
//
//     3 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^5
//     1 <= queries.length <= 10^5
//     queries[i][0] == 1 or queries[i][0] == 2
//     For all i that:
//         queries[i][0] == 1: 0 <= queries[i][1] <= queries[i][2] <= nums.length - 1
//         queries[i][0] == 2: 0 <= queries[i][1] <= nums.length - 1, 1 <= queries[i][2] <= 10^5
//

struct Solution;

impl Solution {
    pub fn count_of_peaks(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();
        let mut m = 1;

        while n > m {
            m <<= 1;
        }

        let mut t = vec![0; 2 * m - 1];

        for i in 0..n {
            if i > 0 && i < n - 1 && nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
                t[i + m - 1] = 1;
            }
        }

        for i in (2..t.len()).step_by(2).rev() {
            let j = i;
            let k = i - 1;

            t[k / 2] = t[j] + t[k];
        }

        let mut r = Vec::new();

        for q in queries {
            if q[0] == 1 {
                r.push(Self::count(q[1] as usize, q[2] as usize, m, &t));
            } else {
                Self::change(q[1] as usize, n, m, q[2], &mut nums, &mut t);
            }
        }

        r
    }

    pub fn count(mut i: usize, mut j: usize, m: usize, t: &[i32]) -> i32 {
        i += m;
        j += m;

        let ml = t[i - 1];
        let mr = if i == j { 0 } else { t[j - 1] };

        let mut s = 0;

        while i <= j {
            if i % 2 == 1 {
                s += t[i - 1];
                i += 1;
            }

            if j.is_multiple_of(2) {
                s += t[j - 1];
                j -= 1;
            }

            i /= 2;
            j /= 2;
        }

        s - ml - mr
    }

    pub fn change(mut k: usize, n: usize, m: usize, x: i32, v: &mut [i32], t: &mut [i32]) {
        v[k] = x;

        if k > 0 && k < n - 1 && v[k] > v[k - 1] && v[k] > v[k + 1] {
            t[k + m - 1] = 1;
        } else {
            t[k + m - 1] = 0;
        }

        if k > 1 && v[k - 1] > v[k - 2] && v[k - 1] > v[k] {
            t[k + m - 2] = 1;
        } else if k > 0 {
            t[k + m - 2] = 0;
        }

        if k < n - 2 && v[k + 1] > v[k] && v[k + 1] > v[k + 2] {
            t[k + m] = 1;
        } else if k < n - 1 {
            t[k + m] = 0;
        }

        let l = k;

        k = l + m;
        k /= 2;

        while k >= 1 {
            t[k - 1] = t[2 * k - 1] + t[2 * k];
            k /= 2;
        }

        if l > 0 {
            k = l + m - 1;
            k /= 2;

            while k >= 1 {
                t[k - 1] = t[2 * k - 1] + t[2 * k];
                k /= 2;
            }
        }

        if l < n - 1 {
            k = l + m + 1;
            k /= 2;

            while k >= 1 {
                t[k - 1] = t[2 * k - 1] + t[2 * k];
                k /= 2;
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![3, 1, 4, 2, 5];
    let queries = vec![vec![2, 3, 4], vec![1, 0, 4]];
    let output = vec![0];
    assert_eq!(Solution::count_of_peaks(nums, queries), output);

    let nums = vec![4, 1, 4, 2, 1, 5];
    let queries = vec![vec![2, 2, 4], vec![1, 0, 2], vec![1, 0, 4]];
    let output = vec![0, 1];
    assert_eq!(Solution::count_of_peaks(nums, queries), output);
}
