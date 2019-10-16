package main

// Check if there are cells without definitive values
func checkSudokuIsComplete(ioManager sudokuIOManager) bool {
	for i := 0; i < 9; i++ {
		for j := 0; j < 9; j++ {
			if len(ioManager.GetSlice(i, j)) > 1 {
				return false
			}
		}
	}
	return true
}

// Create a sudokuIOManager and call the function below
func checkBaseSudokuCorrectness(sudoku [9][9][]int) bool {
	sudokuContainer := newContainer(sudoku)
	ioManager := sudokuIOManager{&sudokuContainer, make(chan sudokuDeleteNumber), make(chan bool), make(chan [9][9][]int)}
	return checkSudokuCorrectness(ioManager)
}

// Check if the cells with definitive values have valid numbers
func checkSudokuCorrectness(ioManager sudokuIOManager) bool {
	// Make channels to communicate the correctness
	horizontalChannel := make(chan bool, 1)
	verticalChannel := make(chan bool, 1)
	boxesChannel := make(chan bool, 1)

	// Start the subroutines to check horizontal, vertical and boxes correctness
	go checkHorizontalCorrectness(ioManager, horizontalChannel)
	go checkVerticalCorrectness(ioManager, verticalChannel)
	go checkBoxesCorrectness(ioManager, boxesChannel)

	// Return the result of the three analysis
	return <-horizontalChannel && <-verticalChannel && <-boxesChannel
}

// Check if all rows are correct
func checkHorizontalCorrectness(ioManager sudokuIOManager, updatesChannel chan bool) {
	correct := true
	for i := 0; i < 9 && correct; i++ {
		// List of already analysed values
		var valuesList []int
		for j := 0; j < 9 && correct; j++ {
			cellSlice := ioManager.GetSlice(i, j)
			// Only cells with definitive values are analysed
			if len(cellSlice) == 1 {
				// If the value is in valuesList then the sudoku is wrong
				if contains(valuesList, cellSlice[0]) {
					correct = false
				} else {
					// Save the value for next checks
					valuesList = append(valuesList, cellSlice[0])
				}
			}
		}
	}
	updatesChannel <- correct
}

// Check if all columns are correct
func checkVerticalCorrectness(ioManager sudokuIOManager, updatesChannel chan bool) {
	correct := true
	for j := 0; j < 9 && correct; j++ {
		// List of already analysed values
		var valuesList []int
		for i := 0; i < 9 && correct; i++ {
			cellSlice := ioManager.GetSlice(i, j)
			// Only cells with definitive values are analysed
			if len(cellSlice) == 1 {
				// If the value is in valuesList then the sudoku is wrong
				if contains(valuesList, cellSlice[0]) {
					correct = false
				} else {
					// Save the value for next checks
					valuesList = append(valuesList, cellSlice[0])
				}
			}
		}
	}
	updatesChannel <- correct
}

// Check if all boxes are correct
func checkBoxesCorrectness(ioManager sudokuIOManager, updatesChannel chan bool) {
	correct := true
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			// List of already analysed values
			var valuesList []int
			for ib := 0; ib < 3; ib++ {
				for jb := 0; jb < 3; jb++ {
					// Get the slice of the analyzed cell
					cellSlice := ioManager.GetSlice(i*3+ib, j*3+jb)
					if len(cellSlice) == 1 {
						// If the value is in valuesList then the sudoku is wrong
						if contains(valuesList, cellSlice[0]) {
							correct = false
						} else {
							// Save the value for next checks
							valuesList = append(valuesList, cellSlice[0])
						}
					}
				}
			}
		}
	}
	updatesChannel <- correct
}
