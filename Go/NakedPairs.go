package main

// Find all the horizontal, vertical and boxes naked pairs
func findNakedPairs(ioManager sudokuIOManager) {
	// Booleans to check if there are deleted values in the following goroutines
	horizontalUpdates := true
	verticalUpdates := true
	boxesUpdates := true
	// Channels used by the goroutines to communicate that there is at least one value modified
	horizontalChannel := make(chan bool, 1)
	verticalChannel := make(chan bool, 1)
	boxesChannel := make(chan bool, 1)
	// While there is at least one goroutine that updates one or more values, the three goroutines must be executed again
	for horizontalUpdates || verticalUpdates || boxesUpdates {
		// Start finding naked pairs threads
		go findHorizontalNakedPairs(ioManager, horizontalChannel)
		go findVerticalNakedPairs(ioManager, verticalChannel)
		go findBoxesNakedPairs(ioManager, boxesChannel)

		// Save the response of every channel in the respective boolean variable
		horizontalUpdates = <-horizontalChannel
		verticalUpdates = <-verticalChannel
		boxesUpdates = <-boxesChannel

		if horizontalUpdates || verticalUpdates || boxesUpdates {
			// If there are deleted values then the constraints must be updated
			constraintsElimination(ioManager)
		}
	}
}

// Find the couples of cells in the same row with two identical possible numbers
func findHorizontalNakedPairs(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	sudokuCopy := ioManager.GetSudoku()
	for i := 0; i < 9; i++ {
		// Save the index of every cell with two elements, in order to minimize the number of analyzed cells
		var validCellsPosition []int
		validCombination := false
		for j := 0; j < 9; j++ {
			if len(sudokuCopy[i][j]) == 2 {
				validCellsPosition = append(validCellsPosition, j)
			}
		}
		// If there are zero or one cells with two possible numbers, there can't be naked pairs in this row
		if len(validCellsPosition) > 1 {
			// Find the combinations of the cells
			combinationsList := getCombinations(validCellsPosition, 2, 0)
			// Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
			for z := 0; z < len(combinationsList) && !validCombination; z++ {
				// Save the indexes of the analyzed cells columns in specific variables for convenience
				firstValuePosition := combinationsList[z][0]
				secondValuePosition := combinationsList[z][1]
				// Check if the two possible values of the first cell of the combination are the same of the second cell
				if contains(sudokuCopy[i][secondValuePosition], sudokuCopy[i][firstValuePosition][0]) &&
					contains(sudokuCopy[i][secondValuePosition], sudokuCopy[i][firstValuePosition][1]) {
					// This is a naked pair, so remove these two values from the other cells of the row
					for j := 0; j < 9; j++ {
						// If the analyzed cell is not one of the combination cells delete the two possible numbers if they belongs to its possible values
						if j != firstValuePosition && j != secondValuePosition &&
							(checkSliceElement(sudokuCopy[i][j], sudokuCopy[i][firstValuePosition][0]) || checkSliceElement(sudokuCopy[i][j], sudokuCopy[i][firstValuePosition][1])) {

							ioManager.DeleteNumber(i, j, sudokuCopy[i][firstValuePosition][0])
							ioManager.DeleteNumber(i, j, sudokuCopy[i][firstValuePosition][1])
							updates = true
							validCombination = true
						}
					}
				}
			}
		}
	}
	updatesChannel <- updates
}

// Find the couples of cells in the same column with two identical possible numbers
func findVerticalNakedPairs(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	sudokuCopy := ioManager.GetSudoku()
	for j := 0; j < 9; j++ {
		// Save the index of every cell with two elements, in order to minimize the number of analyzed cells
		var validCellsPosition []int
		validCombination := false
		for i := 0; i < 9; i++ {
			if len(sudokuCopy[i][j]) == 2 {
				validCellsPosition = append(validCellsPosition, i)
			}
		}
		// If there are zero or one cells with two possible numbers, there can't be naked pairs in this column
		if len(validCellsPosition) > 1 {
			// Find the combinations of the cells
			combinationsList := getCombinations(validCellsPosition, 2, 0)
			// Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
			for z := 0; z < len(combinationsList) && !validCombination; z++ {
				// Save the indexes of the analyzed cells rows in specific variables for convenience
				firstValuePosition := combinationsList[z][0]
				secondValuePosition := combinationsList[z][1]
				// Check if the two possible values of the first cell of the combination are the same of the second cell
				if contains(sudokuCopy[secondValuePosition][j], sudokuCopy[firstValuePosition][j][0]) &&
					contains(sudokuCopy[secondValuePosition][j], sudokuCopy[firstValuePosition][j][1]) {
					// This is a naked pair, so remove these two values from the other cells of the column
					for i := 0; i < 9; i++ {
						// If the analyzed cell is not one of the combination cells delete the two possible numbers (if there isn't a number the requests will be ignored)
						if i != firstValuePosition && i != secondValuePosition &&
							(checkSliceElement(sudokuCopy[i][j], sudokuCopy[firstValuePosition][j][0]) || checkSliceElement(sudokuCopy[i][j], sudokuCopy[firstValuePosition][j][1])) {

							ioManager.DeleteNumber(i, j, sudokuCopy[firstValuePosition][j][0])
							ioManager.DeleteNumber(i, j, sudokuCopy[firstValuePosition][j][1])
							updates = true
							validCombination = true
						}
					}
				}
			}
		}
	}
	updatesChannel <- updates
}

// Find the couples of cells in the same box with two identical possible numbers
func findBoxesNakedPairs(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	sudokuCopy := ioManager.GetSudoku()
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			// Save the index of every cell with two elements, in order to minimize the number of analyzed cells
			var validCellsPosition []int
			validCombination := false
			for ib := 0; ib < 3; ib++ {
				for jb := 0; jb < 3; jb++ {
					if len(sudokuCopy[i*3+ib][j*3+jb]) == 2 {
						// Save the position with this formula to find the row an the column cell using a single number
						validCellsPosition = append(validCellsPosition, ib*3+jb)
					}
				}
			}
			// If there are zero or one cells with two possible numbers, there can't be naked pairs in this box
			if len(validCellsPosition) > 1 {
				// Find the combinations of the cells
				combinationsList := getCombinations(validCellsPosition, 2, 0)
				// Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
				for z := 0; z < len(combinationsList) && !validCombination; z++ {
					// Save the indexes of the analyzed cells boxes in specific variables for convenience
					firstValueRowPosition := combinationsList[z][0] / 3
					firstValueColumnPosition := combinationsList[z][0] % 3
					secondValueRowPosition := combinationsList[z][1] / 3
					secondValueColumnPosition := combinationsList[z][1] % 3
					// Check if the two possible values of the first cell of the combination are the same of the second cell
					if contains(sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][0]) &&
						contains(sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][1]) {
						// This is a naked pair, so remove these two values from the other cells of the box
						for ib := 0; ib < 3; ib++ {
							for jb := 0; jb < 3; jb++ {
								// If the analyzed cell is not one of the combination cells delete the two possible numbers (if there isn't a number the requests will be ignored)
								if (ib != firstValueRowPosition || jb != firstValueColumnPosition) && (ib != secondValueRowPosition || jb != secondValueColumnPosition) &&
									(checkSliceElement(sudokuCopy[i*3+ib][j*3+jb], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][0]) || checkSliceElement(sudokuCopy[i*3+i][j*3+jb], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][1])) {

									ioManager.DeleteNumber(i*3+ib, j*3+jb, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][0])
									ioManager.DeleteNumber(i*3+ib, j*3+jb, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][1])
									updates = true
									validCombination = true
								}
							}
						}
					}
				}
			}
		}
	}
	updatesChannel <- updates
}

// Utility functions for combinations
func getCombinations(arr []int, leng, startPosition int) [][]int {
	result := make([]int, leng)
	return combinations(arr, leng, startPosition, result)
}

// Return all possible combinations of length leng of the elements in arr
func combinations(arr []int, leng, startPosition int, result []int) [][]int {
	var finalResult [][]int
	if leng == 0 {
		// In result there is one combination, which copied and saved in finalResult
		tmp := make([]int, len(result))
		copy(tmp, result)
		return append(finalResult, tmp)
	}
	for i := startPosition; i <= len(arr)-leng; i++ {
		result[len(result)-leng] = arr[i]
		// Append to finalResult the combinations obtained with the recursive calls of this function
		finalResult = append(finalResult, combinations(arr, leng-1, i+1, result)...)
	}
	return finalResult
}

func checkSliceElement(slice []int, value int) bool {
	return len(slice) > 1 && contains(slice, value)
}
