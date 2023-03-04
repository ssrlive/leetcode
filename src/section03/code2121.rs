#![allow(dead_code)]

/*

// 2121. Intervals Between Identical Elements
// https://leetcode.com/problems/intervals-between-identical-elements/
// https://leetcode.cn/problems/intervals-between-identical-elements/
//
// Medium
//
// You are given a 0-indexed array of n integers arr.

The interval between two elements in arr is defined as the absolute difference between their indices. More formally, the interval between arr[i] and arr[j] is |i - j|.

Return an array intervals of length n where intervals[i] is the sum of intervals between arr[i] and each element in arr with the same value as arr[i].

Note: |x| is the absolute value of x.

Example 1:

Input: arr = [2,1,3,1,2,3,3]
Output: [4,2,7,2,4,4,5]
Explanation:
- Index 0: Another 2 is found at index 4. |0 - 4| = 4
- Index 1: Another 1 is found at index 3. |1 - 3| = 2
- Index 2: Two more 3s are found at indices 5 and 6. |2 - 5| + |2 - 6| = 7
- Index 3: Another 1 is found at index 1. |3 - 1| = 2
- Index 4: Another 2 is found at index 0. |4 - 0| = 4
- Index 5: Two more 3s are found at indices 2 and 6. |5 - 2| + |5 - 6| = 4
- Index 6: Two more 3s are found at indices 2 and 5. |6 - 2| + |6 - 5| = 5

Example 2:

Input: arr = [10,5,10,10]
Output: [5,0,3,4]
Explanation:
- Index 0: Two more 10s are found at indices 2 and 3. |0 - 2| + |0 - 3| = 5
- Index 1: There is only one 5 in the array, so its sum of intervals to identical elements is 0.
- Index 2: Two more 10s are found at indices 0 and 3. |2 - 0| + |2 - 3| = 3
- Index 3: Two more 10s are found at indices 0 and 2. |3 - 0| + |3 - 2| = 4

Constraints:

    n == arr.length
    1 <= n <= 10^5
    1 <= arr[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        use std::collections::HashMap;
        let mut val_locs: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, val) in arr.iter().enumerate() {
            val_locs.entry(*val).or_insert(Vec::new()).push(idx);
        }
        let mut ans: Vec<i64> = vec![0; arr.len()];
        for locs in val_locs.values() {
            let mut left: i64 = 0;
            let right: usize = locs.iter().sum();
            let mut right = right as i64;
            let r = locs.len() as i64;
            for (pos, idx) in locs.iter().enumerate() {
                let pos = pos as i64;
                let idx64 = *idx as i64;
                right -= idx64;
                ans[*idx] += right - idx64 * (r - pos - 1);
                ans[*idx] -= left - idx64 * pos;
                left += idx64;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 1, 3, 1, 2, 3, 3], vec![4, 2, 7, 2, 4, 4, 5]),
        (vec![10, 5, 10, 10], vec![5, 0, 3, 4]),
    ];
    for (arr, expected) in cases {
        assert_eq!(Solution::get_distances(arr), expected);
    }
}
