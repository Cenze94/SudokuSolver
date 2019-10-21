use std::fs;
use std::io::{self, Write};

pub struct sudoku {
    // The first 2 dimensions are the rows and the columns of the sudoku, the third contains all the possible values of the cell
    pub sudokuMatrix: [[Vec<i8>; 9]; 9]
}

impl sudoku {
    // Delete a specific value from a specific cell
    pub fn deleteCellValue (&mut self, i:i8, j:i8, value:i8) {
        deleteSliceElement(&mut self.sudokuMatrix[i as usize][j as usize], value)
    }

    // Get the slice of a specific cell
    pub fn getCellValue(&self, i:i8, j:i8) -> Vec<i8> {
        return self.sudokuMatrix[i as usize][j as usize].clone();
    }

    // Check if the cell has more than 2 elements and contains the value given in input
    pub fn checkCellValue(&self, i:i8, j:i8, value:i8) -> bool {
        return self.sudokuMatrix[i as usize][j as usize].len() > 1 && contains(&self.sudokuMatrix[i as usize][j as usize], value);
    }

    // Sudoku constructor, which saves a sudokuMatrix in a new container
    pub fn newContainer(sudokuMatrix:[[Vec<i8>; 9]; 9]) -> sudoku {
        return sudoku{sudokuMatrix};
    }

    // Sudoku constructor, loads the file from the path given in input and initialize the sudoku matrix with its content
    pub fn new(path:String) -> sudoku {
        let matrix: [[Vec<i8>; 9]; 9] = Default::default();
        let mut sudokuVar = sudoku{sudokuMatrix: matrix};

        // Load file in a string
        let fileString = fs::read_to_string(path).unwrap();

        // Row (i) and column (j) indices
        let mut i = 0;
        let mut j = 0;
        // Check every file character
        for z in 0..fileString.len() {
            // Check if the character is a number
            if isNumeric(fileString.chars().nth(z).unwrap()) {
                // Save the value
                let mut vec = Vec::new();
                vec.push(fileString.chars().nth(z).unwrap().to_digit(10).unwrap() as i8);
                sudokuVar.sudokuMatrix[i][j] = vec;
                // Increment column index, if the row is finished then reset j and increment i
                j += 1;
                if j == 9 {
                    i += 1;
                    j = 0;
                }
            } else if fileString.chars().nth(z).unwrap() == '_' {
                // The character is a placeholder for a value to find, so the corresponding vector will contain every possible number
                let possibleNumbersVec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
                sudokuVar.sudokuMatrix[i][j] = possibleNumbersVec;
                // Increment column index, if the row is finished then reset j and increment i
                j += 1;
                if j == 9 {
                    i += 1;
                    j = 0;
                }
            }
        }
        return sudokuVar;
    }

    // This function contains the general structure of sudoku printing, and uses some utility methods
    pub fn printSudoku(&self) {
        for i in 0..9 {
            self.printSudokuRow(i);
            self.printSudokuEndRow();
            // Print a second end row to highlight the end of a line of boxes
            if i == 2 || i == 5 {
                self.printSudokuEndRow();
            }
        }
    }

    // Print a row, dividing the values in 3 lines
    fn printSudokuRow(&self, row:i8) {
        let mut valuesList = Vec::new();
        // Iterate for every column of the sudoku matrix
        for j in 0..9 {
            // Save the values of the analyzed cell in a variable
            let sudokuCell = self.sudokuMatrix[row as usize][j].clone();
            // Check if the value has already been found or not
            if sudokuCell.len() > 1 {
                for z in 1..4 {
                    // If the cell contains z, add the number to the list of the values to print, otherwise add -1 to print a space
                    if contains(&sudokuCell, z) {
                        valuesList.push(z);
                    } else {
                        valuesList.push(-1);
                    }
                }
            } else {
                // The line of this cell will be empty, so three -1 are added to print spaces
                valuesList.push(-1);
                valuesList.push(-1);
                valuesList.push(-1);
            }
            // Signal that the next numbers are referred to a different cell
            valuesList.push(0);
        }
        self.printSudokuLine(valuesList);

        valuesList = Vec::new();
        // Iterate for every column of the sudoku matrix
        for j in 0..9 {
            // Save the values of the analyzed cell in a variable
            let sudokuCell = self.sudokuMatrix[row as usize][j].clone();
            // Check if the value has already been found or not
            if sudokuCell.len() > 1 {
                for z in 4..7 {
                    // If the cell contains z, add the number to the list of the values to print, otherwise add -1 to print a space
                    if contains(&sudokuCell, z) {
                        valuesList.push(z);
                    } else {
                        valuesList.push(-1);
                    }
                }
            } else {
                // The line of this cell will contain only one value in the middle, so two -1 are added to print spaces
                valuesList.push(-1);
                // The value is incremented of 10 to signal that is definitive, and not one of possible values
                valuesList.push(self.sudokuMatrix[row as usize][j][0] + 10);
                valuesList.push(-1);
            }
            // Signal that the next numbers are referred to a different cell
            valuesList.push(0);
        }
        self.printSudokuLine(valuesList);

        valuesList = Vec::new();
        // Iterate for every column of the sudoku matrix
        for j in 0..9 {
            // Save the values of the analyzed cell in a variable
            let sudokuCell = self.sudokuMatrix[row as usize][j].clone();
            // Check if the value has already been found or not
            if sudokuCell.len() > 1 {
                for z in 7..10 {
                    // If the cell contains z, add the number to the list of the values to print, otherwise add -1 to print a space
                    if contains(&sudokuCell, z) {
                        valuesList.push(z);
                    } else {
                        valuesList.push(-1)
                    }
                }
            } else {
                // The line of this cell will be empty, so three -1 are added to print spaces
                valuesList.push(-1);
                valuesList.push(-1);
                valuesList.push(-1);
            }
            // Signal that the next numbers are referred to a different cell
            valuesList.push(0);
        }
        self.printSudokuLine(valuesList);
    }

    // Print the end line of a row
    fn printSudokuEndRow(&self) {
        let mut stringToPrint = "".to_string();
        for j in 0..9 {
            stringToPrint += "------";
            if j < 8 {
                stringToPrint += "--";
            }
            if j == 2 || j == 5 {
                stringToPrint += "-"
            }
        }
        println!("{}", stringToPrint);
    }

    // Print a single line
    fn printSudokuLine(&self, valuesToPrint:Vec<i8>) {
        let mut stringToPrint = "".to_string();
        let mut cellString = "".to_string();
        // Columns index
        let mut numColumns = 0;
        // Check every value of the slice given in input
        for j in 0..valuesToPrint.len() {
            // Check if the value in not 0
            if valuesToPrint[j] != 0 {
                // If the value is definitive then has been incremented of 10
                if valuesToPrint[j] > 10 {
                    cellString += &((valuesToPrint[j] - 10).to_string() + " ");
                } else if valuesToPrint[j] == -1 {
                    // Print a space because the corresponding value is missing
                    cellString += "  ";
                } else {
                    // Print the value
                    cellString += &(valuesToPrint[j].to_string() + " ");
                }
            } else {
                // Save the cell string into the string to print, add the chars to signal the end of the cell and the end of the box
                stringToPrint += &cellString;
                if numColumns == 2 || numColumns == 5 {
                    stringToPrint += "|";
                }
                if numColumns < 8 {
                    stringToPrint += "| ";
                }
                // Empty the cell string and increment the columns index
                cellString = "".to_string();
                numColumns += 1;
            }
        }
        println!("{}", stringToPrint);
    }
}

// Utility function, check if a slice contains a specific value
pub fn contains(slice: &Vec<i8>, v: i8) -> bool {
    return slice.contains(&v);
}

// Utility function, delete a specific value from a slice
pub fn deleteSliceElement(slice: &mut Vec<i8>, v: i8) {
    let pos = findSliceElement(&slice, v);
    slice.remove(pos);
}

// Utility function, return the position of a specific value in a slice
pub fn findSliceElement(slice: &Vec<i8>, v: i8) -> usize {
    return slice.iter().position(|&r| r == v).unwrap();
}

// Utility function, check if a character is a number
pub fn isNumeric(b: char) -> bool {
    return b.is_digit(10);
}