#![allow(dead_code)]

// 3636. Threshold Majority Queries
// https://leetcode.com/problems/threshold-majority-queries/
// https://leetcode.cn/problems/threshold-majority-queries/
//
// Hard
//
// You are given an integer array nums of length n and an array queries, where queries[i] = [li, ri, thresholdi].
//
// Return an array of integers ans where ans[i] is equal to the element in the subarray nums[li...ri]
// that appears at least thresholdi times, selecting the element with the highest frequency
// (choosing the smallest in case of a tie), or -1 if no such element exists.
//
// Example 1:
//
// Input: nums = [1,1,2,2,1,1], queries = [[0,5,4],[0,3,3],[2,3,2]]
//
// Output: [1,-1,2]
//
// Explanation:
// Query	Sub-array	Threshold	Frequency table	Answer
// [0, 5, 4]	[1, 1, 2, 2, 1, 1]	4	1 → 4, 2 → 2	1
// [0, 3, 3]	[1, 1, 2, 2]	3	1 → 2, 2 → 2	-1
// [2, 3, 2]	[2, 2]	2	2 → 2	2
//
// Example 2:
//
// Input: nums = [3,2,3,2,3,2,3], queries = [[0,6,4],[1,5,2],[2,4,1],[3,3,1]]
//
// Output: [3,2,3,2]
//
// Explanation:
// Query	Sub-array	Threshold	Frequency table	Answer
// [0, 6, 4]	[3, 2, 3, 2, 3, 2, 3]	4	3 → 4, 2 → 3	3
// [1, 5, 2]	[2, 3, 2, 3, 2]	2	2 → 3, 3 → 2	2
// [2, 4, 1]	[3, 2, 3]	1	3 → 2, 2 → 1	3
// [3, 3, 1]	[2]	1	2 → 1	2
//
// Constraints:
//
//     1 <= nums.length == n <= 10^4
//     1 <= nums[i] <= 10^9
//     1 <= queries.length <= 5 * 10^4
//     queries[i] = [li, ri, thresholdi]
//     0 <= li <= ri < n
//     1 <= thresholdi <= ri - li + 1
//

struct Solution;

impl Solution {
    pub fn subarray_majority(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();
        let mut a: Vec<i32> = nums.clone();
        a.sort_unstable();
        a.dedup();
        let m = a.len();
        let mut p = std::collections::BTreeMap::new();
        for (i, &x) in a.iter().enumerate() {
            p.insert(x, i as i32);
        }
        for x in &mut nums {
            *x = *p.get(x).unwrap();
        }
        let mut cnt = vec![vec![0; m]; n + 1];
        for i in 1..=n {
            cnt[i] = cnt[i - 1].clone();
            cnt[i][nums[i - 1] as usize] += 1;
        }
        let q = queries.len();
        let mut ans = vec![-1; q];
        for i in 0..q {
            let mut hf = 0;
            let mut v = -1;
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            let th = queries[i][2];
            for (j, (&r_cnt, &l_cnt)) in cnt[r + 1].iter().zip(&cnt[l]).enumerate() {
                let freq = r_cnt - l_cnt;
                if freq > hf {
                    hf = freq;
                    v = j as i32;
                }
            }
            if hf >= th {
                ans[i] = a[v as usize];
            }
        }

        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 2, 1, 1];
    let queries = vec![vec![0, 5, 4], vec![0, 3, 3], vec![2, 3, 2]];
    let expected = vec![1, -1, 2];
    assert_eq!(Solution::subarray_majority(nums, queries), expected);

    let nums = vec![3, 2, 3, 2, 3, 2, 3];
    let queries = vec![vec![0, 6, 4], vec![1, 5, 2], vec![2, 4, 1], vec![3, 3, 1]];
    let expected = vec![3, 2, 3, 2];
    assert_eq!(Solution::subarray_majority(nums, queries), expected);
}
