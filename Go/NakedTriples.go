package main

// Find all the horizontal, vertical and boxes naked triples
func findNakedTriples(ioManager sudokuIOManager) {
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
		// Start finding naked triples threads
		go findHorizontalNakedTriples(ioManager, horizontalChannel)
		go findVerticalNakedTriples(ioManager, verticalChannel)
		go findBoxesNakedTriples(ioManager, boxesChannel)

		// Save the response of every channel in the respective boolean variable
		horizontalUpdates = <-horizontalChannel
		verticalUpdates = <-verticalChannel
		boxesUpdates = <-boxesChannel

		if horizontalUpdates || verticalUpdates || boxesUpdates {
			// If there are deleted values then the constraints must be updated
			constraintsElimination(ioManager)
			// After the elimination of the values there could be naked pairs
			findNakedPairs(ioManager)
		}
	}
}

// Find the triples of cells in the same row with two or three identical possible numbers
func findHorizontalNakedTriples(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	sudokuCopy := ioManager.GetSudoku()
	for i := 0; i < 9; i++ {
		// Save the index of every cell with two or three elements, in order to minimize the number of analyzed cells
		var validCellsPosition []int
		validCombination := false
		for j := 0; j < 9; j++ {
			if len(sudokuCopy[i][j]) == 2 || len(sudokuCopy[i][j]) == 3 {
				validCellsPosition = append(validCellsPosition, j)
			}
		}
		// If there are less than three cells with two or three possible numbers, there can't be naked triples in this row
		if len(validCellsPosition) > 2 {
			// Find the combinations of the cells
			combinationsList := getCombinations(validCellsPosition, 3, 0)
			// Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
			for z := 0; z < len(combinationsList) && !validCombination; z++ {
				// Save the indexes of the analyzed cells columns in specific variables for convenience
				firstValuePosition := combinationsList[z][0]
				secondValuePosition := combinationsList[z][1]
				thirdValuePosition := combinationsList[z][2]
				// Get the union of the possible values of the analyzed cells
				var values []int
				for t := 0; t < 2; t++ {
					if !contains(values, sudokuCopy[i][firstValuePosition][t]) {
						values = append(values, sudokuCopy[i][firstValuePosition][t])
					}
					if !contains(values, sudokuCopy[i][secondValuePosition][t]) {
						values = append(values, sudokuCopy[i][secondValuePosition][t])
					}
					if !contains(values, sudokuCopy[i][thirdValuePosition][t]) {
						values = append(values, sudokuCopy[i][thirdValuePosition][t])
					}
				}
				// The analyzed cells could not have the third element
				slice := sudokuCopy[i][firstValuePosition]
				if len(slice) == 3 && !contains(values, slice[2]) {
					values = append(values, sudokuCopy[i][firstValuePosition][2])
				}
				slice = sudokuCopy[i][secondValuePosition]
				if len(slice) == 3 && !contains(values, slice[2]) {
					values = append(values, sudokuCopy[i][secondValuePosition][2])
				}
				slice = ioManager.GetSlice(i, thirdValuePosition)
				if len(slice) == 3 && !contains(values, slice[2]) {
					values = append(values, sudokuCopy[i][thirdValuePosition][2])
				}
				// If the size of the union is three then the analyzed cells are a naked triple
				if len(values) == 3 {
					// Remove these values from the other cells of the row
					for j := 0; j < 9; j++ {
						if j != firstValuePosition && j != secondValuePosition && j != thirdValuePosition &&
							(checkSliceElement(sudokuCopy[i][j], values[0]) || checkSliceElement(sudokuCopy[i][j], values[1]) || checkSliceElement(sudokuCopy[i][j], values[2])) {

							ioManager.DeleteNumber(i, j, values[0])
							ioManager.DeleteNumber(i, j, values[1])
							ioManager.DeleteNumber(i, j, values[2])
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

// Find the triples of cells in the same column with two or three identical possible numbers
func findVerticalNakedTriples(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	sudokuCopy := ioManager.GetSudoku()
	for j := 0; j < 9; j++ {
		// Save the index of every cell with two or three elements, in order to minimize the number of analyzed cells
		var validCellsPosition []int
		validCombination := false
		for i := 0; i < 9; i++ {
			if len(sudokuCopy[i][j]) == 2 || len(sudokuCopy[i][j]) == 3 {
				validCellsPosition = append(validCellsPosition, i)
			}
		}
		// If there are less than three cells with two or three possible numbers, there can't be naked triples in this column
		if len(validCellsPosition) > 2 {
			// Find the combinations of the cells
			combinationsList := getCombinations(validCellsPosition, 3, 0)
			// Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
			for z := 0; z < len(combinationsList) && !validCombination; z++ {
				// Save the indexes of the analyzed cells rows in specific variables for convenience
				firstValuePosition := combinationsList[z][0]
				secondValuePosition := combinationsList[z][1]
				thirdValuePosition := combinationsList[z][2]
				// Get the union of the possible values of the analyzed cells
				var values []int
				for t := 0; t < 2; t++ {
					if !contains(values, sudokuCopy[firstValuePosition][j][t]) {
						values = append(values, sudokuCopy[firstValuePosition][j][t])
					}
					if !contains(values, sudokuCopy[secondValuePosition][j][t]) {
						values = append(values, sudokuCopy[secondValuePosition][j][t])
					}
					if !contains(values, sudokuCopy[thirdValuePosition][j][t]) {
						values = append(values, sudokuCopy[thirdValuePosition][j][t])
					}
				}
				// The analyzed cells could not have the third element
				slice := sudokuCopy[firstValuePosition][j]
				if len(slice) == 3 && !contains(values, slice[2]) {
					values = append(values, sudokuCopy[firstValuePosition][j][2])
				}
				slice = sudokuCopy[secondValuePosition][j]
				if len(slice) == 3 && !contains(values, slice[2]) {
					values = append(values, sudokuCopy[secondValuePosition][j][2])
				}
				slice = sudokuCopy[thirdValuePosition][j]
				if len(slice) == 3 && !contains(values, slice[2]) {
					values = append(values, sudokuCopy[thirdValuePosition][j][2])
				}
				// If the size of the union is three then the analyzed cells are a naked triple
				if len(values) == 3 {
					// Remove these values from the other cells of the column
					for i := 0; i < 9; i++ {
						if i != firstValuePosition && i != secondValuePosition && i != thirdValuePosition &&
							(checkSliceElement(sudokuCopy[i][j], values[0]) || checkSliceElement(sudokuCopy[i][j], values[1]) || checkSliceElement(sudokuCopy[i][j], values[2])) {

							ioManager.DeleteNumber(i, j, values[0])
							ioManager.DeleteNumber(i, j, values[1])
							ioManager.DeleteNumber(i, j, values[2])
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

// Find the triples of cells in the same box with two or three identical possible numbers
func findBoxesNakedTriples(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	sudokuCopy := ioManager.GetSudoku()
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			// Save the index of every cell with two or three elements, in order to minimize the number of analyzed cells
			var validCellsPosition []int
			validCombination := false
			for ib := 0; ib < 3; ib++ {
				for jb := 0; jb < 3; jb++ {
					if len(sudokuCopy[i*3+ib][j*3+jb]) == 2 || len(sudokuCopy[i*3+ib][j*3+jb]) == 3 {
						// Save the position with this formula to find the row an the column cell using a single number
						validCellsPosition = append(validCellsPosition, ib*3+jb)
					}
				}
			}
			// If there are less than three cells with two or three possible numbers, there can't be naked triples in this box
			if len(validCellsPosition) > 2 {
				// Find the combinations of the cells
				combinationsList := getCombinations(validCellsPosition, 3, 0)
				// Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
				for z := 0; z < len(combinationsList) && !validCombination; z++ {
					// Save the indexes of the analyzed cells rows and columns in specific variables for convenience
					firstValueRowPosition := combinationsList[z][0] / 3
					firstValueColumnPosition := combinationsList[z][0] % 3
					secondValueRowPosition := combinationsList[z][1] / 3
					secondValueColumnPosition := combinationsList[z][1] % 3
					thirdValueRowPosition := combinationsList[z][2] / 3
					thirdValueColumnPosition := combinationsList[z][2] % 3
					// Get the union of the possible values of the analyzed cells
					var values []int
					for t := 0; t < 2; t++ {
						if !contains(values, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][t]) {
							values = append(values, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][t])
						}
						if !contains(values, sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition][t]) {
							values = append(values, sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition][t])
						}
						if !contains(values, sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition][t]) {
							values = append(values, sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition][t])
						}
					}
					// The analyzed cells could not have the third element
					slice := sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition]
					if len(slice) == 3 && !contains(values, slice[2]) {
						values = append(values, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][2])
					}
					slice = sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition]
					if len(slice) == 3 && !contains(values, slice[2]) {
						values = append(values, sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition][2])
					}
					slice = sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition]
					if len(slice) == 3 && !contains(values, slice[2]) {
						values = append(values, sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition][2])
					}
					// If the size of the union is three then the analyzed cells are a naked triple
					if len(values) == 3 {
						// Remove these values from the other cells of the box
						for ib := 0; ib < 3; ib++ {
							for jb := 0; jb < 3; jb++ {
								if (ib != firstValueRowPosition || jb != firstValueColumnPosition) && (ib != secondValueRowPosition || jb != secondValueColumnPosition) && (ib != thirdValueRowPosition || jb != thirdValueColumnPosition) &&
									(checkSliceElement(sudokuCopy[i*3+ib][j*3+jb], values[0]) || checkSliceElement(sudokuCopy[i*3+ib][j*3+jb], values[1]) || checkSliceElement(sudokuCopy[i*3+ib][j*3+jb], values[2])) {

									ioManager.DeleteNumber(i*3+ib, j*3+jb, values[0])
									ioManager.DeleteNumber(i*3+ib, j*3+jb, values[1])
									ioManager.DeleteNumber(i*3+ib, j*3+jb, values[2])
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
