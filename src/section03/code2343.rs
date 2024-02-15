#![allow(dead_code)]

/*

// 2343. Query Kth Smallest Trimmed Number
// https://leetcode.com/problems/query-kth-smallest-trimmed-number/
// https://leetcode.cn/problems/query-kth-smallest-trimmed-number/
//
// Medium
//
// You are given a 0-indexed array of strings nums, where each string is of equal length and consists of only digits.

You are also given a 0-indexed 2D integer array queries where queries[i] = [ki, trimi]. For each queries[i], you need to:

    Trim each number in nums to its rightmost trimi digits.
    Determine the index of the kith smallest trimmed number in nums. If two trimmed numbers are equal, the number with the lower index is considered to be smaller.
    Reset each number in nums to its original length.

Return an array answer of the same length as queries, where answer[i] is the answer to the ith query.

Note:

    To trim to the rightmost x digits means to keep removing the leftmost digit, until only x digits remain.
    Strings in nums may contain leading zeros.

Example 1:

Input: nums = ["102","473","251","814"], queries = [[1,1],[2,3],[4,2],[1,2]]
Output: [2,2,1,0]
Explanation:
1. After trimming to the last digit, nums = ["2","3","1","4"]. The smallest number is 1 at index 2.
2. Trimmed to the last 3 digits, nums is unchanged. The 2nd smallest number is 251 at index 2.
3. Trimmed to the last 2 digits, nums = ["02","73","51","14"]. The 4th smallest number is 73.
4. Trimmed to the last 2 digits, the smallest number is 2 at index 0.
   Note that the trimmed number "02" is evaluated as 2.

Example 2:

Input: nums = ["24","37","96","04"], queries = [[2,1],[2,2]]
Output: [3,0]
Explanation:
1. Trimmed to the last digit, nums = ["4","7","6","4"]. The 2nd smallest number is 4 at index 3.
   There are two occurrences of 4, but the one at index 0 is considered smaller than the one at index 3.
2. Trimmed to the last 2 digits, nums is unchanged. The 2nd smallest number is 24.

Constraints:

    1 <= nums.length <= 100
    1 <= nums[i].length <= 100
    nums[i] consists of only digits.
    All nums[i].length are equal.
    1 <= queries.length <= 100
    queries[i].length == 2
    1 <= ki <= nums.length
    1 <= trimi <= nums[i].length

Follow up: Could you use the Radix Sort Algorithm to solve this problem? What will be the complexity of that solution?
*/

struct Solution;

impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn get_kth_num(nums: &[String], trim: usize, k: usize) -> i32 {
            let len_ns: usize = nums.len();
            let mut res: Vec<(&str, usize)> = Vec::with_capacity(len_ns);
            for (idx, num) in nums.iter().enumerate() {
                let len_n: usize = num.len();
                res.push((&num[len_n - trim..], idx));
            }
            res.sort_by_key(|&r| r.0);
            res[k - 1].1 as i32
        }

        let len_qs: usize = queries.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_qs);
        for query in queries {
            let k: usize = query[0] as usize;
            let trim: usize = query[1] as usize;
            let res = get_kth_num(&nums, trim, k);
            ans.push(res);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["102", "473", "251", "814"],
            vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]],
            vec![2, 2, 1, 0],
        ),
        (vec!["24", "37", "96", "04"], vec![vec![2, 1], vec![2, 2]], vec![3, 0]),
    ];
    for (nums, queries, expect) in cases {
        let nums: Vec<String> = nums.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::smallest_trimmed_numbers(nums, queries), expect);
    }
}
