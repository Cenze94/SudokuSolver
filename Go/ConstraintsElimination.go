package main

import (
	"fmt"
	"strconv"
	"time"
)

// Create a sudokuIOManager and call the function below
func sudokuConstraintsElimination(sudoku [9][9][]int) {
	sudokuContainer := newContainer(sudoku)
	newManager := sudokuIOManager{&sudokuContainer, make(chan sudokuDeleteNumber), make(chan bool), make(chan [9][9][]int)}
	// Run the manager to delete the values sent in the specific channel
	go newManager.Run()
	constraintsElimination(newManager)
}

// Delete all the horizontal, vertical and boxes invalid constraints
func constraintsElimination(ioManager sudokuIOManager) {
	start := time.Now().UnixNano()
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
		// Start constraint elimination threads
		go horizontalConstraintElimination(ioManager, horizontalChannel)
		go verticalConstraintElimination(ioManager, verticalChannel)
		go boxesConstraintElimination(ioManager, boxesChannel)

		// Save the response of every channel in the respective boolean variable
		horizontalUpdates = <-horizontalChannel
		verticalUpdates = <-verticalChannel
		boxesUpdates = <-boxesChannel
	}
	fmt.Println("ConstraintsElimination time: " + strconv.FormatInt((time.Now().UnixNano()-start)/int64(time.Microsecond), 10))
}

// Delete horizontal constraints, updates signals if there are deleted values
func horizontalConstraintElimination(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	for i := 0; i < 9; i++ {
		for j := 0; j < 9; j++ {
			// Get the slice of the analyzed cell
			cellSlice := ioManager.GetSlice(i, j)
			// If the slice has a definitive value then delete the occurences of the same value in the row
			if len(cellSlice) == 1 {
				cellValue := cellSlice[0]
				// Delete the value for the previous cells without definitive values
				for z := 0; z < j; z++ {
					if len(ioManager.GetSlice(i, z)) > 1 && ioManager.CheckNumber(i, z, cellValue) {
						ioManager.DeleteNumber(i, z, cellValue)
						updates = true
					}
				}
				// Delete the value for the next cells without definitive values
				for z := j + 1; z < 9; z++ {
					if len(ioManager.GetSlice(i, z)) > 1 && ioManager.CheckNumber(i, z, cellValue) {
						ioManager.DeleteNumber(i, z, cellValue)
						updates = true
					}
				}
			}
		}
	}
	updatesChannel <- updates
}

// Delete vertical constraints, updates signals if there are deleted values
func verticalConstraintElimination(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	for j := 0; j < 9; j++ {
		for i := 0; i < 9; i++ {
			// Get the slice of the analyzed cell
			cellSlice := ioManager.GetSlice(i, j)
			// If the slice has a definitive value then delete the occurences of the same value in the column
			if len(cellSlice) == 1 {
				cellValue := cellSlice[0]
				// Delete the value for the previous cells without definitive values
				for z := 0; z < i; z++ {
					if len(ioManager.GetSlice(z, j)) > 1 && ioManager.CheckNumber(z, j, cellValue) {
						ioManager.DeleteNumber(z, j, cellValue)
						updates = true
					}
				}
				// Delete the value for the next cells without definitive values
				for z := i + 1; z < 9; z++ {
					if len(ioManager.GetSlice(z, j)) > 1 && ioManager.CheckNumber(z, j, cellValue) {
						ioManager.DeleteNumber(z, j, cellValue)
						updates = true
					}
				}
			}
		}
	}
	updatesChannel <- updates
}

// Delete box constraints, updates signals if there are deleted values
func boxesConstraintElimination(ioManager sudokuIOManager, updatesChannel chan bool) {
	updates := false
	for i := 0; i < 9; i++ {
		for j := 0; j < 9; j++ {
			// Get the slice of the analyzed cell
			cellSlice := ioManager.GetSlice(i, j)
			// If the slice has a definitive value then delete the occurences of the same value in the box
			if len(cellSlice) == 1 {
				cellValue := cellSlice[0]
				// Get the position of the analyzed box among the other boxes
				var boxRowPosition int = i / 3
				var boxColumnPosition int = j / 3
				// Update the cells of the box
				for ib := boxRowPosition * 3; ib < boxRowPosition*3+3; ib++ {
					for jb := boxColumnPosition * 3; jb < boxColumnPosition*3+3; jb++ {
						// Delete the value if the cell doesn't have a definitive value (note that the
						// original cell is excluded automatically because it has a definitive value)
						if len(ioManager.GetSlice(ib, jb)) > 1 && ioManager.CheckNumber(ib, jb, cellValue) {
							ioManager.DeleteNumber(ib, jb, cellValue)
							updates = true
						}
					}
				}
			}
		}
	}
	updatesChannel <- updates
}
