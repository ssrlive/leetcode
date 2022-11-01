#![allow(dead_code)]

// 278. First Bad Version
// https://leetcode.com/problems/first-bad-version/
//
// You are a product manager and currently leading a team to develop a new
// product. Unfortunately, the latest version of your product fails the quality
// check. Since each version is developed based on the previous version, all the
// versions after a bad version are also bad.
//
// Suppose you have n versions [1, 2, ..., n] and you want to find out the first
// bad one, which causes all the following ones to be bad.
//
// You are given an API bool isBadVersion(version) which will return whether
// version is bad. Implement a function to find the first bad version. You
// should minimize the number of calls to the API.
//
// Example:
//
// Given n = 5, and version = 4 is the first bad version.
//
// call isBadVersion(3) -> false
// call isBadVersion(5) -> true
// call isBadVersion(4) -> true
//
// Then 4 is the first bad version.
//

struct Solution {
    first_bad_version: i32,
}

impl Solution {
    fn new(first_bad_version: i32) -> Self {
        Solution { first_bad_version }
    }

    #[allow(non_snake_case)]
    pub fn isBadVersion(&self, version: i32) -> bool {
        version >= self.first_bad_version
    }
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[test]
fn test_first_bad_version() {
    let s = Solution::new(4);
    assert_eq!(s.first_bad_version(5), 4);
}
