package main

// Contain the variables needed to delete a specific value from a given cell
type sudokuDeleteNumber struct {
	row, column, value int
}

// Manager of sudoku output channels
type sudokuIOManager struct {
	sudokuVar     *sudoku
	delete        chan sudokuDeleteNumber
	requestSudoku chan bool
	sendSudoku    chan [9][9][]int
}

// Run method of sudokuIOManager, for every value to delete check if the number exists (because of the
// execution of multiple concurrent threads)
func (self *sudokuIOManager) Run() {
	for {
		select {
		// Check if there is a request of a copy of the sudoku
		case <-self.requestSudoku:
			var copy [9][9][]int
			for i := 0; i < 9; i++ {
				for j := 0; j < 9; j++ {
					for z := 0; z < len(self.sudokuVar.sudokuMatrix[i][j]); z++ {
						copy[i][j] = append(copy[i][j], self.sudokuVar.sudokuMatrix[i][j][z])
					}
				}
			}
			self.sendSudoku <- copy
		// Otherwise wait and delete the next value
		case deleteNumber := <-self.delete:
			if self.sudokuVar.checkCellValue(deleteNumber.row, deleteNumber.column, deleteNumber.value) {
				self.sudokuVar.deleteCellValue(deleteNumber.row, deleteNumber.column, deleteNumber.value)
			}
		}
	}
}

// This method adds the data about the number to delete to the delete channel
func (self *sudokuIOManager) DeleteNumber(i, j, value int) {
	self.delete <- sudokuDeleteNumber{i, j, value}
}

// This method returns the slice of the cell in the position given in input.
// The consequence is that every thread will work with the IOManager instead of the sudoku
func (self *sudokuIOManager) GetSlice(i, j int) []int {
	return self.sudokuVar.getCellValue(i, j)
}

// This method check if the given value is contained in the cell in position i and j
func (self *sudokuIOManager) CheckNumber(i, j, value int) bool {
	return self.sudokuVar.checkCellValue(i, j, value)
}

// Return the sudoku for the brute force methods
func (self *sudokuIOManager) GetSudoku() [9][9][]int {
	self.requestSudoku <- true
	return <-self.sendSudoku
}

func (self *sudokuIOManager) PrintSudoku() {
	self.sudokuVar.printSudoku()
}
