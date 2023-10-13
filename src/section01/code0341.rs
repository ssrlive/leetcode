#![allow(dead_code)]

// 341. Flatten Nested List Iterator
// https://leetcode.com/problems/flatten-nested-list-iterator/
// https://leetcode.cn/problems/flatten-nested-list-iterator/
//
// You are given a nested list of integers nestedList. Each element is either an integer or a list
// whose elements may also be integers or other lists. Implement an iterator to flatten it.
//
// Implement the NestedIterator class:
//
// - NestedIterator(List<NestedInteger> nestedList) Initializes the iterator with the nested list nestedList.
// - int next() Returns the next integer in the nested list.
// - boolean hasNext() Returns true if there are still some integers in the nested list and false otherwise.
//
// Your code will be tested with the following pseudocode:
//
//   initialize iterator with nestedList
//   res = []
//   while iterator.hasNext()
//       append iterator.next() to the end of res
//   return res
//
// If res matches the expected flattened list, then your code will be judged as correct.
//
// Example 1:
//
// Input: nestedList = [[1,1],2,[1,1]]
// Output: [1,1,2,1,1]
// Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,1,2,1,1].
//
// Example 2:
//
// Input: nestedList = [1,[4,[6]]]
// Output: [1,4,6]
// Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,4,6].
//
// Constraints:
//
// - 1 <= nestedList.length <= 500
// - The values of the integers in the nested list is in the range [-10^6, 10^6].
//

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

#[derive(Debug, Clone)]
pub struct NestedIterator {
    stack: Vec<NestedInteger>,
    next: Option<i32>,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut s = Self {
            stack: nested_list.into_iter().rev().collect::<Vec<_>>(),
            next: None,
        };
        s.advance_next();
        s
    }

    fn next(&mut self) -> i32 {
        let ret = self.next.unwrap();
        self.advance_next();
        ret
    }

    fn has_next(&mut self) -> bool {
        self.next.is_some()
    }

    fn advance_next(&mut self) {
        while let Some(last) = self.stack.pop() {
            match last {
                NestedInteger::Int(n) => {
                    self.next = Some(n);
                    return;
                }
                NestedInteger::List(list) => {
                    self.stack.extend(list.into_iter().rev());
                }
            }
        }
        self.next = None;
    }
}

#[test]
fn test() {
    let nested_list = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    let mut iter = NestedIterator::new(nested_list);
    assert_eq!(iter.next(), 1);
    assert_eq!(iter.next(), 1);
    assert_eq!(iter.next(), 2);
    assert_eq!(iter.next(), 1);
    assert_eq!(iter.next(), 1);
    assert!(!iter.has_next());
}
