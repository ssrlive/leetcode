#![allow(dead_code)]

// 3484. Design Spreadsheet
// https://leetcode.com/problems/design-spreadsheet/
// https://leetcode.cn/problems/design-spreadsheet/
//
// Medium
//
// A spreadsheet is a grid with 26 columns (labeled from 'A' to 'Z') and a given number of rows. Each cell in the spreadsheet can hold an integer value between 0 and 105.
//
// Implement the Spreadsheet class:
//
// - Spreadsheet(int rows) Initializes a spreadsheet with 26 columns (labeled 'A' to 'Z') and the specified number of rows. All cells are initially set to 0.
// - void setCell(String cell, int value) Sets the value of the specified cell. The cell reference is provided in the format
//   "AX" (e.g., "A1", "B10"), where the letter represents the column (from 'A' to 'Z') and the number represents a 1-indexed row.
// - void resetCell(String cell) Resets the specified cell to 0.
// - int getValue(String formula) Evaluates a formula of the form "=X+Y", where X and Y are either cell references or non-negative integers, and returns the computed sum.
//
// Note: If getValue references a cell that has not been explicitly set using setCell, its value is considered 0.
//
// Example 1:
//
// Input:
// ["Spreadsheet", "getValue", "setCell", "getValue", "setCell", "getValue", "resetCell", "getValue"]
// [[3], ["=5+7"], ["A1", 10], ["=A1+6"], ["B2", 15], ["=A1+B2"], ["A1"], ["=A1+B2"]]
//
// Output:
// [null, 12, null, 16, null, 25, null, 15]
//
// Explanation
// Spreadsheet spreadsheet = new Spreadsheet(3); // Initializes a spreadsheet with 3 rows and 26 columns
// spreadsheet.getValue("=5+7"); // returns 12 (5+7)
// spreadsheet.setCell("A1", 10); // sets A1 to 10
// spreadsheet.getValue("=A1+6"); // returns 16 (10+6)
// spreadsheet.setCell("B2", 15); // sets B2 to 15
// spreadsheet.getValue("=A1+B2"); // returns 25 (10+15)
// spreadsheet.resetCell("A1"); // resets A1 to 0
// spreadsheet.getValue("=A1+B2"); // returns 15 (0+15)
//
// Constraints:
//
// - 1 <= rows <= 10^3
// - 0 <= value <= 10^5
// - The formula is always in the format "=X+Y", where X and Y are either valid cell references or non-negative
//   integers with values less than or equal to 105.
// - Each cell reference consists of a capital letter from 'A' to 'Z' followed by a row number between 1 and rows.
// - At most 104 calls will be made in total to setCell, resetCell, and getValue.
//

struct Spreadsheet {
    sp: Vec<Vec<i32>>,
}

impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        Self {
            sp: vec![vec![0; 26]; rows as usize + 1],
        }
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        let col = cell.chars().nth(0).unwrap() as usize - 'A' as usize;
        let row = cell[1..].parse::<usize>().unwrap();
        self.sp[row][col] = value;
    }

    pub fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    pub fn get_value(&self, formula: String) -> i32 {
        let formula = &formula[1..];
        let plus = formula.find('+').unwrap();
        let x = self._get_val(&formula[..plus]);
        let y = self._get_val(&formula[plus + 1..]);
        x + y
    }

    fn _get_val(&self, cell: &str) -> i32 {
        if cell.chars().nth(0).unwrap().is_ascii_digit() {
            cell.parse::<i32>().unwrap()
        } else {
            let col = cell.chars().nth(0).unwrap() as usize - 'A' as usize;
            let row = cell[1..].parse::<usize>().unwrap();
            self.sp[row][col]
        }
    }
}

#[test]
fn test() {
    let mut spreadsheet = Spreadsheet::new(3);
    assert_eq!(spreadsheet.get_value("=5+7".to_string()), 12);
    spreadsheet.set_cell("A1".to_string(), 10);
    assert_eq!(spreadsheet.get_value("=A1+6".to_string()), 16);
    spreadsheet.set_cell("B2".to_string(), 15);
    assert_eq!(spreadsheet.get_value("=A1+B2".to_string()), 25);
    spreadsheet.reset_cell("A1".to_string());
    assert_eq!(spreadsheet.get_value("=A1+B2".to_string()), 15);
}
