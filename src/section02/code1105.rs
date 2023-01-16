#![allow(dead_code)]

// 1105. Filling Bookcase Shelves
// https://leetcode.com/problems/filling-bookcase-shelves/
// https://leetcode.cn/problems/filling-bookcase-shelves/
//
// You are given an array books where books[i] = [thicknessi, heighti] indicates the thickness and height of the ith book. You are also given an integer shelfWidth.
//
// We want to place these books in order onto bookcase shelves that have a total width shelfWidth.
//
// We choose some of the books to place on this shelf such that the sum of their thickness is less than or equal to shelfWidth,
// then build another level of the shelf of the bookcase so that the total height of the bookcase has increased by the maximum
// height of the books we just put down. We repeat this process until there are no more books to place.
//
// Note that at each step of the above process, the order of the books we place is the same order as the given sequence of books.
//
// For example, if we have an ordered list of 5 books, we might place the first and second book onto the first shelf,
// the third book on the second shelf, and the fourth and fifth book on the last shelf.
//
// Return the minimum possible height that the total bookshelf can be after placing shelves in this manner.
//
// Example 1:
//
// Input: books = [[1,1],[2,3],[2,3],[1,1],[1,1],[1,1],[1,2]], shelf_width = 4
// Output: 6
// Explanation:
// The sum of the heights of the 3 shelves is 1 + 3 + 2 = 6.
// Notice that book number 2 does not have to be on the first shelf.
//
// Example 2:
//
// Input: books = [[1,3],[2,4],[3,2]], shelfWidth = 6
// Output: 4
//
// Constraints:
//
// - 1 <= books.length <= 1000
// - 1 <= thicknessi <= shelfWidth <= 1000
// - 1 <= heighti <= 1000
//

struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut heights = vec![i32::MAX / 2; books.len()];
        *heights.first_mut().unwrap() = *books.first().and_then(|book| book.last()).unwrap();

        for i in 0..books.len() {
            let mut remaining_width = shelf_width;
            let mut max_height_this_level = 0;

            for j in (0..=i).rev() {
                let book = books.get(j).unwrap();
                let book_width: i32 = *book.first().unwrap();

                if remaining_width < book_width {
                    break;
                }
                remaining_width -= book_width;

                max_height_this_level = std::cmp::max(max_height_this_level, *book.last().unwrap());

                let prev_height: i32 = if j == 0 { 0 } else { *heights.get(j - 1).unwrap_or(&0) };
                *heights.get_mut(i).unwrap() =
                    std::cmp::min(*heights.get(i).unwrap(), prev_height + max_height_this_level);
            }
        }

        *heights.last().unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![1, 1],
                vec![2, 3],
                vec![2, 3],
                vec![1, 1],
                vec![1, 1],
                vec![1, 1],
                vec![1, 2],
            ],
            4,
            6,
        ),
        (vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6, 4),
    ];
    for (books, shelf_width, expected) in cases {
        assert_eq!(Solution::min_height_shelves(books, shelf_width), expected);
    }
}
