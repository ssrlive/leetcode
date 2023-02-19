#![allow(dead_code)]

// 2570. Merge Two 2D Arrays by Summing Values
// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/
// https://leetcode.cn/problems/merge-two-2d-arrays-by-summing-values/
//
// Easy
//
// You are given two 2D integer arrays nums1 and nums2.
//
// -    nums1[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.
// -    nums2[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.
//
// Each array contains unique ids and is sorted in ascending order by id.
//
// Merge the two arrays into one array that is sorted in ascending order by id, respecting the following conditions:
//
// -    Only ids that appear in at least one of the two arrays should be included in the resulting array.
// -    Each id should be included only once and its value should be the sum of the values of this id in the two arrays. If the id does not exist in one of the two arrays then its value in that array is considered to be 0.
//
// Return the resulting array. The returned array must be sorted in ascending order by id.
//
// Example 1:
//
// Input: nums1 = [[1,2],[2,3],[4,5]], nums2 = [[1,4],[3,2],[4,1]]
// Output: [[1,6],[2,3],[3,2],[4,6]]
// Explanation: The resulting array contains the following:
// - id = 1, the value of this id is 2 + 4 = 6.
// - id = 2, the value of this id is 3.
// - id = 3, the value of this id is 2.
// - id = 4, the value of this id is 5 + 1 = 6.
//
// Example 2:
//
// Input: nums1 = [[2,4],[3,6],[5,5]], nums2 = [[1,3],[4,3]]
// Output: [[1,3],[2,4],[3,6],[4,3],[5,5]]
// Explanation: There are no common ids, so we just include each id with its value in the resulting list.
//
// Constraints:
//
// -    1 <= nums1.length, nums2.length <= 200
// -    nums1[i].length == nums2[j].length == 2
// -    1 <= idi, vali <= 1000
// -    Both arrays contain unique ids.
// -    Both arrays are in strictly ascending order by id.
//

struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let mut result = Vec::new();

        while !nums1.is_empty() && !nums2.is_empty() {
            let id1 = nums1[0][0];
            let id2 = nums2[0][0];
            match id1.cmp(&id2) {
                std::cmp::Ordering::Less => {
                    result.push(nums1[0].clone());
                    nums1.remove(0);
                }
                std::cmp::Ordering::Equal => {
                    result.push(vec![id1, nums1[0][1] + nums2[0][1]]);
                    nums1.remove(0);
                    nums2.remove(0);
                }
                std::cmp::Ordering::Greater => {
                    result.push(nums2[0].clone());
                    nums2.remove(0);
                }
            }
        }

        result.extend(nums1);
        result.extend(nums2);

        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            vec![vec![1, 4], vec![3, 2], vec![4, 1]],
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]],
        ),
        (
            vec![vec![2, 4], vec![3, 6], vec![5, 5]],
            vec![vec![1, 3], vec![4, 3]],
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]],
        ),
    ];
    for (nums1, nums2, expected) in cases {
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected);
    }
}
