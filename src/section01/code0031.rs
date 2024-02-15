#![allow(dead_code)]

// 31. Next Permutation
// https://leetcode.com/problems/next-permutation/
// https://leetcode.cn/problems/next-permutation/
//
// A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
//
// - For example, for arr = [1,2,3], the following are all the permutations of arr: [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
//
// The next permutation of an array of integers is the next lexicographically greater permutation of its integer. More formally,
// if all the permutations of the array are sorted in one container according to their lexicographical order,
// then the next permutation of that array is the permutation that follows it in the sorted container.
// If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).
//
// - For example, the next permutation of arr = [1,2,3] is [1,3,2].
// - Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
// - While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.
//
// Given an array of integers nums, find the next permutation of nums.
//
// The replacement must be in place and use only constant extra memory.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: [1,3,2]
//
// Example 2:
//
// Input: nums = [3,2,1]
// Output: [1,2,3]
//
// Example 3:
//
// Input: nums = [1,1,5]
// Output: [1,5,1]
//
// Constraints:
//
// - 1 <= nums.length <= 100
// - 0 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        let l = nums.len();
        let (mut i, mut j) = (l - 1, l - 1);

        // get the index of last increasing subseq elem from right
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        // nums[i-1] is the smallest in range (i-1..len-1)
        // elems from (i..len - 1) are in descending
        if i > 0 {
            // get index of the first element >= nums[i-1]
            // equivalent to sorting the element in ascending and get the index of element right after nums[i-1]
            while j >= i && nums[j] <= nums[i - 1] {
                j -= 1;
            }
            // swap the smallest and next greater element
            nums.swap(i - 1, j);
        }

        // reverse the elements from (i to len - 1) to convert to ascending
        nums[i..l].reverse();
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 3, 2]);

    let mut nums = vec![3, 2, 1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 2, 3]);

    let mut nums = vec![1, 1, 5];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 5, 1]);

    let mut nums = vec![1, 3, 2];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![2, 1, 3]);

    let mut nums = vec![1, 5, 1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![5, 1, 1]);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 9, 8]);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 9, 8];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 8, 7, 9]);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 8, 7, 9];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 8, 9, 7]);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 8, 9, 7];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 9, 7, 8]);
}
