#![allow(dead_code)]

/*

// 1948. Delete Duplicate Folders in System
// https://leetcode.com/problems/delete-duplicate-folders-in-system/
// https://leetcode.cn/problems/delete-duplicate-folders-in-system/
//
// Hard
//
// Due to a bug, there are many duplicate folders in a file system. You are given a 2D array paths, where paths[i] is an array representing an absolute path to the ith folder in the file system.

    For example, ["one", "two", "three"] represents the path "/one/two/three".

Two folders (not necessarily on the same level) are identical if they contain the same non-empty set of identical subfolders and underlying subfolder structure. The folders do not need to be at the root level to be identical. If two or more folders are identical, then mark the folders as well as all their subfolders.

    For example, folders "/a" and "/b" in the file structure below are identical. They (as well as their subfolders) should all be marked:
        /a
        /a/x
        /a/x/y
        /a/z
        /b
        /b/x
        /b/x/y
        /b/z
    However, if the file structure also included the path "/b/w", then the folders "/a" and "/b" would not be identical. Note that "/a/x" and "/b/x" would still be considered identical even with the added folder.

Once all the identical folders and their subfolders have been marked, the file system will delete all of them. The file system only runs the deletion once, so any folders that become identical after the initial deletion are not deleted.

Return the 2D array ans containing the paths of the remaining folders after deleting all the marked folders. The paths may be returned in any order.

Example 1:

Input: paths = [["a"],["c"],["d"],["a","b"],["c","b"],["d","a"]]
Output: [["d"],["d","a"]]
Explanation: The file structure is as shown.
Folders "/a" and "/c" (and their subfolders) are marked for deletion because they both contain an empty
folder named "b".

Example 2:

Input: paths = [["a"],["c"],["a","b"],["c","b"],["a","b","x"],["a","b","x","y"],["w"],["w","y"]]
Output: [["c"],["c","b"],["a"],["a","b"]]
Explanation: The file structure is as shown.
Folders "/a/b/x" and "/w" (and their subfolders) are marked for deletion because they both contain an empty folder named "y".
Note that folders "/a" and "/c" are identical after the deletion, but they are not deleted because they were not marked beforehand.

Example 3:

Input: paths = [["a","b"],["c","d"],["c"],["a"]]
Output: [["c"],["c","d"],["a"],["a","b"]]
Explanation: All folders are unique in the file system.
Note that the returned array can be in a different order as the order does not matter.

Constraints:

    1 <= paths.length <= 2 * 10^4
    1 <= paths[i].length <= 500
    1 <= paths[i][j].length <= 10
    1 <= sum(paths[i][j].length) <= 2 * 10^5
    path[i][j] consists of lowercase English letters.
    No two paths lead to the same folder.
    For any folder not at the root level, its parent folder will also be in the input.
*/

struct Solution;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    name: String,
    next: BTreeMap<String, Option<Rc<RefCell<Node>>>>,
    del: bool,
}

impl Node {
    fn new(name: String) -> Self {
        Self {
            name,
            next: BTreeMap::new(),
            del: false,
        }
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut ans = vec![];
        let root = Some(Rc::new(RefCell::new(Node::new("".to_string()))));
        for path in paths {
            Self::add_path(root.clone(), path);
        }
        let mut seen = BTreeMap::new();
        Self::dedupe(root.clone(), &mut seen);

        let mut path = vec![];
        for (_, next) in root.as_ref().unwrap().borrow().next.iter() {
            Self::get_path(next.clone(), &mut ans, &mut path);
        }

        ans
    }

    fn add_path(mut node: Option<Rc<RefCell<Node>>>, path: Vec<String>) {
        for s in path {
            if node.as_ref().unwrap().borrow().next.get(&s).is_none() {
                node.as_mut()
                    .unwrap()
                    .borrow_mut()
                    .next
                    .insert(s.clone(), Some(Rc::new(RefCell::new(Node::new(s.clone())))));
            }
            let tmp = node.as_ref().unwrap().borrow().next.get(&s).unwrap().clone();
            node = tmp;
        }
    }

    fn dedupe(mut node: Option<Rc<RefCell<Node>>>, seen: &mut BTreeMap<String, Option<Rc<RefCell<Node>>>>) -> String {
        let mut subfolder = String::new();
        for (_, next) in node.as_ref().unwrap().borrow().next.iter() {
            subfolder += &Self::dedupe(next.clone(), seen);
        }
        if !subfolder.is_empty() {
            if seen.contains_key(&subfolder) {
                seen.get_mut(&subfolder).unwrap().as_mut().unwrap().borrow_mut().del = true;
                node.as_mut().unwrap().borrow_mut().del = true;
            } else {
                seen.insert(subfolder.clone(), node.clone());
            }
        }
        format!("({}{})", node.as_ref().unwrap().borrow().name, subfolder)
    }

    fn get_path(node: Option<Rc<RefCell<Node>>>, ans: &mut Vec<Vec<String>>, path: &mut Vec<String>) {
        if node.as_ref().unwrap().borrow().del {
            return;
        }
        path.push(node.as_ref().unwrap().borrow().name.clone());
        ans.push(path.clone());
        for (_, next) in node.as_ref().unwrap().borrow().next.iter() {
            Self::get_path(next.clone(), ans, path);
        }
        path.pop();
    }
}

#[test]
fn test() {
    let paths = vec![
        vec!["a".to_string()],
        vec!["c".to_string()],
        vec!["d".to_string()],
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string(), "b".to_string()],
        vec!["d".to_string(), "a".to_string()],
    ];
    let ans = vec![vec!["d".to_string()], vec!["d".to_string(), "a".to_string()]];
    assert_eq!(Solution::delete_duplicate_folder(paths), ans);
    let paths = vec![
        vec!["a".to_string()],
        vec!["c".to_string()],
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string(), "b".to_string()],
        vec!["a".to_string(), "b".to_string(), "x".to_string()],
        vec!["a".to_string(), "b".to_string(), "x".to_string(), "y".to_string()],
        vec!["w".to_string()],
        vec!["w".to_string(), "y".to_string()],
    ];
    let ans = vec![
        vec!["a".to_string()],
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string()],
        vec!["c".to_string(), "b".to_string()],
    ];
    assert_eq!(Solution::delete_duplicate_folder(paths), ans);
    let paths = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string(), "d".to_string()],
        vec!["c".to_string()],
        vec!["a".to_string()],
    ];
    let ans = vec![
        vec!["a".to_string()],
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string()],
        vec!["c".to_string(), "d".to_string()],
    ];
    assert_eq!(Solution::delete_duplicate_folder(paths), ans);
}
