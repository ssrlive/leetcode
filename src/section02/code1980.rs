#![allow(dead_code)]

/*

// 1980. Find Unique Binary String
// https://leetcode.com/problems/find-unique-binary-string/
// https://leetcode.cn/problems/find-unique-binary-string/
//
// Medium
//
// Given an array of strings nums containing n unique binary strings each of length n, return a binary string of length n that does not appear in nums. If there are multiple answers, you may return any of them.

Example 1:

Input: nums = ["01","10"]
Output: "11"
Explanation: "11" does not appear in nums. "00" would also be correct.

Example 2:

Input: nums = ["00","01"]
Output: "11"
Explanation: "11" does not appear in nums. "10" would also be correct.

Example 3:

Input: nums = ["111","011","001"]
Output: "101"
Explanation: "101" does not appear in nums. "000", "010", "100", and "110" would also be correct.

Constraints:

    n == nums.length
    1 <= n <= 16
    nums[i].length == n
    nums[i] is either '0' or '1'.
    All the strings of nums are unique.
*/

struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let len = nums[0].len();
        let limit = 1 << len;
        let set = nums.into_iter().collect::<Vec<String>>();
        for i in 0..limit {
            let mut arr = vec![" "; len];
            for (j, arr_j) in arr.iter_mut().enumerate() {
                if i >> j & 1 == 1 {
                    *arr_j = "1";
                } else {
                    *arr_j = "0";
                }
            }
            arr.reverse();
            let v = arr.join("");
            if !set.contains(&v) {
                return v;
            }
        }
        unreachable!()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["01", "10"], "00"),
        (vec!["00", "01"], "10"),
        (vec!["111", "011", "001"], "000"),
    ];
    for (nums, expect) in cases {
        let nums = nums.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::find_different_binary_string(nums), expect);
    }
}
