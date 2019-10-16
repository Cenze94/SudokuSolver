package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
)

type sudoku struct {
	// The first 2 dimensions are the rows and the columns of the sudoku, the third contains all the possible values of the cell
	sudokuMatrix [9][9][]int
}

// Delete a specific value from a specific cell
func (self *sudoku) deleteCellValue(i, j, value int) {
	(*self).sudokuMatrix[i][j] = deleteSliceElement((*self).sudokuMatrix[i][j], value)
}

// Get the slice of a specific cell
func (self *sudoku) getCellValue(i, j int) []int {
	return (*self).sudokuMatrix[i][j]
}

// Check if the cell has more than 2 elements and contains the value given in input
func (self *sudoku) checkCellValue(i, j, value int) bool {
	return len((*self).sudokuMatrix[i][j]) > 1 &&
		contains((*self).sudokuMatrix[i][j], value)
}

// Sudoku constructor, which saves a sudokuMatrix in a new container
func newContainer(sudokuMatrix [9][9][]int) sudoku {
	return sudoku{sudokuMatrix}
}

// Sudoku constructor, loads the file from the path given in input and initialize the sudoku matrix with its content
func new(path string) sudoku {
	var sudokuVar sudoku = sudoku{}

	// Load file
	file, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}
	// Save the file content in a string
	fileString := string(file)

	// Row (i) and column (j) indices
	i := 0
	j := 0
	// Check every file character
	for z := 0; z < len(fileString); z++ {
		// Check if the character is a number
		if isNumeric(fileString[z]) {
			// Save the value with a trick
			sudokuVar.sudokuMatrix[i][j] = []int{int(rune(fileString[z]) - '0')}
			// Increment column index, if the row is finished then reset j and increment i
			j++
			if j == 9 {
				i++
				j = 0
			}
		} else if fileString[z] == '_' {
			// The character is a placeholder for a value to find, so the corresponding slice will contain every possible number
			sudokuVar.sudokuMatrix[i][j] = []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
			// Increment column index, if the row is finished then reset j and increment i
			j++
			if j == 9 {
				i++
				j = 0
			}
		}
	}

	return sudokuVar
}

// This function contains the general structure of sudoku printing, and uses some utility methods
func (self *sudoku) printSudoku() {
	for i := 0; i < 9; i++ {
		self.printSudokuRow(i)
		self.printSudokuEndRow()
		// Print a second end row to highlight the end of a line of boxes
		if i == 2 || i == 5 {
			self.printSudokuEndRow()
		}
	}
}

// Print a row, dividing the values in 3 lines
func (self *sudoku) printSudokuRow(row int) {
	var valuesList []int
	// Iterate for every column of the sudoku matrix
	for j := 0; j < 9; j++ {
		// Save the values of the analyzed cell in a variable
		var sudokuCell = (*self).sudokuMatrix[row][j]
		// Check if the value has already been found or not
		if len(sudokuCell) > 1 {
			for z := 1; z < 4; z++ {
				// If the cell contains z, add the number to the list of the values to print, otherwise add -1 to print a space
				if contains(sudokuCell, z) {
					valuesList = append(valuesList, z)
				} else {
					valuesList = append(valuesList, -1)
				}
			}
		} else {
			// The line of this cell will be empty, so three -1 are added to print spaces
			valuesList = append(valuesList, -1)
			valuesList = append(valuesList, -1)
			valuesList = append(valuesList, -1)
		}
		// Signal that the next numbers are referred to a different cell
		valuesList = append(valuesList, 0)
	}
	self.printSudokuLine(valuesList)

	valuesList = nil
	// Iterate for every column of the sudoku matrix
	for j := 0; j < 9; j++ {
		// Save the values of the analyzed cell in a variable
		var sudokuCell = (*self).sudokuMatrix[row][j]
		// Check if the value has already been found or not
		if len(sudokuCell) > 1 {
			for z := 4; z < 7; z++ {
				// If the cell contains z, add the number to the list of the values to print, otherwise add -1 to print a space
				if contains(sudokuCell, z) {
					valuesList = append(valuesList, z)
				} else {
					valuesList = append(valuesList, -1)
				}
			}
		} else {
			// The line of this cell will contain only one value in the middle, so two -1 are added to print spaces
			valuesList = append(valuesList, -1)
			// The value is incremented of 10 to signal that is definitive, and not one of possible values
			valuesList = append(valuesList, (*self).sudokuMatrix[row][j][0]+10)
			valuesList = append(valuesList, -1)
		}
		// Signal that the next numbers are referred to a different cell
		valuesList = append(valuesList, 0)
	}
	self.printSudokuLine(valuesList)

	valuesList = nil
	// Iterate for every column of the sudoku matrix
	for j := 0; j < 9; j++ {
		// Save the values of the analyzed cell in a variable
		var sudokuCell = (*self).sudokuMatrix[row][j]
		// Check if the value has already been found or not
		if len(sudokuCell) > 1 {
			for z := 7; z < 10; z++ {
				// If the cell contains z, add the number to the list of the values to print, otherwise add -1 to print a space
				if contains(sudokuCell, z) {
					valuesList = append(valuesList, z)
				} else {
					valuesList = append(valuesList, -1)
				}
			}
		} else {
			// The line of this cell will be empty, so three -1 are added to print spaces
			valuesList = append(valuesList, -1)
			valuesList = append(valuesList, -1)
			valuesList = append(valuesList, -1)
		}
		// Signal that the next numbers are referred to a different cell
		valuesList = append(valuesList, 0)
	}
	self.printSudokuLine(valuesList)
}

// Print the end line of a row
func (self *sudoku) printSudokuEndRow() {
	var stringToPrint string
	for j := 0; j < 9; j++ {
		stringToPrint += "------"
		if j < 8 {
			stringToPrint += "--"
		}
		if j == 2 || j == 5 {
			stringToPrint += "-"
		}
	}
	fmt.Println(stringToPrint)
}

// Print a single line
func (self *sudoku) printSudokuLine(valuesToPrint []int) {
	var stringToPrint, cellString string
	// Columns index
	var numColumns = 0
	// Check every value of the slice given in input
	for j := 0; j < len(valuesToPrint); j++ {
		// Check if the value in not 0
		if valuesToPrint[j] != 0 {
			// If the value is definitive then has been incremented of 10
			if valuesToPrint[j] > 10 {
				cellString += strconv.Itoa(valuesToPrint[j]-10) + " "
			} else if valuesToPrint[j] == -1 {
				// Print a space because the corresponding value is missing
				cellString += "  "
			} else {
				// Print the value
				cellString += strconv.Itoa(valuesToPrint[j]) + " "
			}
		} else {
			// Save the cell string into the string to print, add the chars to signal the end of the cell and the end of the box
			stringToPrint += cellString
			if numColumns == 2 || numColumns == 5 {
				stringToPrint += "|"
			}
			if numColumns < 8 {
				stringToPrint += "| "
			}
			// Empty the cell string and increment the columns index
			cellString = ""
			numColumns++
		}
	}

	fmt.Println(stringToPrint)
}

// Utility function, check if a slice contains a specific value
func contains(slice []int, v int) bool {
	for _, a := range slice {
		if a == v {
			return true
		}
	}
	return false
}

// Utility function, delete a specific value from a slice
func deleteSliceElement(slice []int, v int) []int {
	pos := findSliceElement(slice, v)
	return append(slice[:pos], slice[pos+1:]...)
}

// Utility function, return the position of a specific value in a slice
func findSliceElement(slice []int, v int) int {
	for z := 0; z < len(slice); z++ {
		if slice[z] == v {
			return z
		}
	}
	return -1
}

// Utility function, check if a character is a number
func isNumeric(b byte) bool {
	return b == '0' || b == '1' || b == '2' || b == '3' || b == '4' || b == '5' || b == '6' ||
		b == '7' || b == '8' || b == '9'
}
