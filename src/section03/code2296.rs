#![allow(dead_code)]

/*

// 2296. Design a Text Editor
// https://leetcode.com/problems/design-a-text-editor/
// https://leetcode.cn/problems/design-a-text-editor/
//
// Hard
//
// Design a text editor with a cursor that can do the following:

    Add text to where the cursor is.
    Delete text from where the cursor is (simulating the backspace key).
    Move the cursor either left or right.

When deleting text, only characters to the left of the cursor will be deleted. The cursor will also remain within the actual text and cannot be moved beyond it. More formally, we have that 0 <= cursor.position <= currentText.length always holds.

Implement the TextEditor class:

    TextEditor() Initializes the object with empty text.
    void addText(string text) Appends text to where the cursor is. The cursor ends to the right of text.
    int deleteText(int k) Deletes k characters to the left of the cursor. Returns the number of characters actually deleted.
    string cursorLeft(int k) Moves the cursor to the left k times. Returns the last min(10, len) characters to the left of the cursor, where len is the number of characters to the left of the cursor.
    string cursorRight(int k) Moves the cursor to the right k times. Returns the last min(10, len) characters to the left of the cursor, where len is the number of characters to the left of the cursor.

Example 1:

Input
["TextEditor", "addText", "deleteText", "addText", "cursorRight", "cursorLeft", "deleteText", "cursorLeft", "cursorRight"]
[[], ["leetcode"], [4], ["practice"], [3], [8], [10], [2], [6]]
Output
[null, null, 4, null, "etpractice", "leet", 4, "", "practi"]

Explanation
TextEditor textEditor = new TextEditor(); // The current text is "|". (The '|' character represents the cursor)
textEditor.addText("leetcode"); // The current text is "leetcode|".
textEditor.deleteText(4); // return 4
                          // The current text is "leet|".
                          // 4 characters were deleted.
textEditor.addText("practice"); // The current text is "leetpractice|".
textEditor.cursorRight(3); // return "etpractice"
                           // The current text is "leetpractice|".
                           // The cursor cannot be moved beyond the actual text and thus did not move.
                           // "etpractice" is the last 10 characters to the left of the cursor.
textEditor.cursorLeft(8); // return "leet"
                          // The current text is "leet|practice".
                          // "leet" is the last min(10, 4) = 4 characters to the left of the cursor.
textEditor.deleteText(10); // return 4
                           // The current text is "|practice".
                           // Only 4 characters were deleted.
textEditor.cursorLeft(2); // return ""
                          // The current text is "|practice".
                          // The cursor cannot be moved beyond the actual text and thus did not move.
                          // "" is the last min(10, 0) = 0 characters to the left of the cursor.
textEditor.cursorRight(6); // return "practi"
                           // The current text is "practi|ce".
                           // "practi" is the last min(10, 6) = 6 characters to the left of the cursor.

Constraints:

    1 <= text.length, k <= 40
    text consists of lowercase English letters.
    At most 2 * 10^4 calls in total will be made to addText, deleteText, cursorLeft and cursorRight.

Follow-up: Could you find a solution with time complexity of O(k) per call?
*/

struct TextEditor {
    left: Vec<char>,
    right: Vec<char>,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            left: vec![],
            right: vec![],
        }
    }

    fn add_text(&mut self, text: String) {
        for c in text.chars() {
            self.left.push(c);
        }
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let n = k.min(self.left.len() as i32);
        for _ in 0..n {
            self.left.pop();
        }
        n
    }

    fn cursor_left(&mut self, k: i32) -> String {
        let n = k.min(self.left.len() as i32);
        for _ in 0..n {
            let c = self.left.pop().unwrap();
            self.right.push(c);
        }
        let slice = &self.left[0.max(self.left.len() as i32 - 10) as usize..];
        slice.iter().collect::<String>()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let n = k.min(self.right.len() as i32);
        for _ in 0..n {
            let c = self.right.pop().unwrap();
            self.left.push(c);
        }
        self.cursor_left(0)
    }
}

#[test]
fn test() {
    let mut text_editor = TextEditor::new();
    text_editor.add_text("leetcode".to_string());
    assert_eq!(text_editor.delete_text(4), 4);
    text_editor.add_text("practice".to_string());
    assert_eq!(text_editor.cursor_right(3), "etpractice");
    assert_eq!(text_editor.cursor_left(8), "leet");
    assert_eq!(text_editor.delete_text(10), 4);
    assert_eq!(text_editor.cursor_left(2), "");
    assert_eq!(text_editor.cursor_right(6), "practi");
}
