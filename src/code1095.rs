#![allow(dead_code)]

// 1095. Find in Mountain Array
// https://leetcode.com/problems/find-in-mountain-array/
// https://leetcode.cn/problems/find-in-mountain-array/
//
// (This problem is an interactive problem.)
//
// You may recall that an array arr is a mountain array if and only if:
//
// arr.length >= 3
// There exists some i with 0 < i < arr.length - 1 such that:
// arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
// arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
// Given a mountain array mountainArr, return the minimum index such that mountainArr.get(index) == target. If such an index does not exist, return -1.
//
// You cannot access the mountain array directly. You may only access the array using a MountainArray interface:
//
// MountainArray.get(k) returns the element of the array at index k (0-indexed).
// MountainArray.length() returns the length of the array.
// Submissions making more than 100 calls to MountainArray.get will be judged Wrong Answer.
//   Also, any solutions that attempt to circumvent the judge will result in disqualification.
//
// Example 1:
//
// Input: array = [1,2,3,4,5,3,1], target = 3
// Output: 2
// Explanation: 3 exists in the array, at index=2 and index=5. Return the minimum index, which is 2.
//
// Example 2:
//
// Input: array = [0,1,2,4,2,1], target = 3
// Output: -1
// Explanation: 3 does not exist in the array, so we return -1.
//
// Constraints:
//
// - 3 <= mountain_arr.length() <= 10^4
// - 0 <= target <= 10^9
// - 0 <= mountain_arr.get(index) <= 10^9
//

struct MountainArray;

impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        // unimplemented!()
        index
    }
    fn length(&self) -> i32 {
        // unimplemented!()
        0
    }
}

struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let mut st = 0;
        let mut end = mountainArr.length() - 1;
        let mut mid = 0;
        while end > st {
            mid = st + (end - st) / 2;
            match mountainArr.get(mid).cmp(&mountainArr.get(mid + 1)) {
                std::cmp::Ordering::Greater => end = mid,
                std::cmp::Ordering::Less => st = mid + 1,
                std::cmp::Ordering::Equal => (),
            }
        }
        let mut _std = mid;
        let mut endd = mountainArr.length() - 1;
        end = st;
        st = 0;
        while st <= end {
            mid = st + (end - st) / 2;
            match mountainArr.get(mid).cmp(&target) {
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Greater => end = mid - 1,
                std::cmp::Ordering::Less => st = mid + 1,
            }
        }
        while _std <= endd {
            mid = _std + (endd - _std) / 2;
            match mountainArr.get(mid).cmp(&target) {
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Less => endd = mid - 1,
                std::cmp::Ordering::Greater => _std = mid + 1,
            }
        }
        -1
    }
}

#[test]
fn test() {
    let target = 3;
    let mountain_arr = MountainArray;
    let result = Solution::find_in_mountain_array(target, &mountain_arr);
    println!("result = {}", result);
}
