#![allow(dead_code)]

/*

// 2179. Count Good Triplets in an Array
// https://leetcode.com/problems/count-good-triplets-in-an-array/
// https://leetcode.cn/problems/count-good-triplets-in-an-array/
//
// Hard
//
// You are given two 0-indexed arrays nums1 and nums2 of length n, both of which are permutations of [0, 1, ..., n - 1].

A good triplet is a set of 3 distinct values which are present in increasing order by position both in nums1 and nums2. In other words, if we consider pos1v as the index of the value v in nums1 and pos2v as the index of the value v in nums2, then a good triplet will be a set (x, y, z) where 0 <= x, y, z <= n - 1, such that pos1x < pos1y < pos1z and pos2x < pos2y < pos2z.

Return the total number of good triplets.

Example 1:

Input: nums1 = [2,0,1,3], nums2 = [0,1,2,3]
Output: 1
Explanation:
There are 4 triplets (x,y,z) such that pos1x < pos1y < pos1z. They are (2,0,1), (2,0,3), (2,1,3), and (0,1,3).
Out of those triplets, only the triplet (0,1,3) satisfies pos2x < pos2y < pos2z. Hence, there is only 1 good triplet.

Example 2:

Input: nums1 = [4,0,1,3,2], nums2 = [4,1,0,2,3]
Output: 4
Explanation: The 4 good triplets are (4,0,3), (4,0,2), (4,1,3), and (4,1,2).

Constraints:

    n == nums1.length == nums2.length
    3 <= n <= 10^5
    0 <= nums1[i], nums2[i] <= n - 1
    nums1 and nums2 are permutations of [0, 1, ..., n - 1].
*/

struct Solution;

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        fn update(u: usize, left: usize, right: usize, i: usize, tree: &mut Vec<i32>) {
            if left == right {
                tree[u] = 1;
                return;
            }
            let mid = left + (right - left) / 2;
            if i <= mid {
                update(2 * u, left, mid, i, tree);
            } else {
                update(2 * u + 1, mid + 1, right, i, tree);
            }
            tree[u] = tree[2 * u] + tree[2 * u + 1];
        }

        fn count(u: usize, left: usize, right: usize, l: usize, r: usize, tree: &Vec<i32>) -> i64 {
            if left >= l && right <= r {
                return tree[u] as i64;
            }
            if left > r || right < l {
                return 0;
            }
            let mid = left + (right - left) / 2;
            count(2 * u, left, mid, l, r, tree) + count(2 * u + 1, mid + 1, right, l, r, tree)
        }

        let n = nums1.len();
        let mut nums2 = nums2;
        let mut mp = vec![0i32; n];
        for i in 0..n {
            mp[nums1[i] as usize] = i as i32;
        }
        for i in 0..n {
            nums2[i] = mp[nums2[i] as usize];
        }
        let mut ret = 0i64;
        let mut tree = vec![0i32; 4 * n];
        for (i, &num2) in nums2.iter().enumerate() {
            if num2 > 0 {
                let smaller_counter = count(1, 0, n - 1, 0, num2 as usize - 1, &tree);
                let bigger_counter = n as i64 - 1 - num2 as i64 - (i as i64 - smaller_counter);
                ret += smaller_counter * bigger_counter;
            }
            update(1, 0, n - 1, num2 as usize, &mut tree);
        }
        ret
    }
}

#[test]
fn test() {
    let nums1 = vec![2, 0, 1, 3];
    let nums2 = vec![0, 1, 2, 3];
    assert_eq!(Solution::good_triplets(nums1, nums2), 1);
    let nums1 = vec![4, 0, 1, 3, 2];
    let nums2 = vec![4, 1, 0, 2, 3];
    assert_eq!(Solution::good_triplets(nums1, nums2), 4);
}
