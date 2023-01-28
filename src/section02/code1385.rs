#![allow(dead_code)]

// 1385. Find the Distance Value Between Two Arrays
// https://leetcode.com/problems/find-the-distance-value-between-two-arrays/
// https://leetcode.cn/problems/find-the-distance-value-between-two-arrays/
//
// Easy
//
// Given two integer arrays arr1 and arr2, and the integer d, return the distance value between the two arrays.
//
// The distance value is defined as the number of elements arr1[i] such that there is not
// any element arr2[j] where |arr1[i]-arr2[j]| <= d.
//
// Example 1:
//
// Input: arr1 = [4,5,8], arr2 = [10,9,1,8], d = 2
// Output: 2
// Explanation:
// For arr1[0]=4 we have:
// |4-10|=6 > d=2
// |4-9|=5 > d=2
// |4-1|=3 > d=2
// |4-8|=4 > d=2
// For arr1[1]=5 we have:
// |5-10|=5 > d=2
// |5-9|=4 > d=2
// |5-1|=4 > d=2
// |5-8|=3 > d=2
// For arr1[2]=8 we have:
// |8-10|=2 <= d=2
// |8-9|=1 <= d=2
// |8-1|=7 > d=2
// |8-8|=0 <= d=2
//
// Example 2:
//
// Input: arr1 = [1,4,2,3], arr2 = [-4,-3,6,10,20,30], d = 3
// Output: 2
//
// Example 3:
//
// Input: arr1 = [2,1,100,3], arr2 = [-5,-2,10,-3,7], d = 6
// Output: 1
//
// Constraints:
//
// -    1 <= arr1.length, arr2.length <= 500
// -    -1000 <= arr1[i], arr2[j] <= 1000
// -    0 <= d <= 100
//

struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut count = 0;
        for i in arr1 {
            let mut flag = true;
            for j in &arr2 {
                if (i - j).abs() <= d {
                    flag = false;
                    break;
                }
            }
            if flag {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 5, 8], vec![10, 9, 1, 8], 2, 2),
        (vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3, 2),
        (vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6, 1),
    ];
    for (arr1, arr2, d, expected) in cases {
        assert_eq!(Solution::find_the_distance_value(arr1, arr2, d), expected);
    }
}
