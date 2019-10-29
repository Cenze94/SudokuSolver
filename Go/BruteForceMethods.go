package main

import (
	"math/big"
)

// Get the solution using brute force, creating a specific channel and calling "searchSolution" as a goroutine
func bruteForceSolving(ioManager sudokuIOManager) sudokuIOManager {
	solutionChannel := make(chan [9][9][]int)
	// Create a channel to send a stop signal to the active goroutines. The size is arbitrary, its purpose is to make the channel non-blocking
	stopChannel := make(chan bool, 100)
	go searchSolution(ioManager.GetSudoku(), solutionChannel, stopChannel)
	sudokuSolution := <-solutionChannel
	// Send a stop signal to the running goroutines, because there is already a correct solution
	stopChannel <- true

	// Save and return the obtained solution in a new sudokuIOManager
	sudokuContainer := newContainer(sudokuSolution)
	newManager := sudokuIOManager{&sudokuContainer, make(chan sudokuDeleteNumber), make(chan bool), make(chan [9][9][]int)}
	// Run the manager to delete the values sent in the specific channel
	go newManager.Run()
	return newManager
}

// Recursive function, called as a subroutine in order to parallelize the search
func searchSolution(sudoku [9][9][]int, solutionChannel chan [9][9][]int, stopChannel chan bool) {
	i := 0
	j := 0
	// Find the first cell without a definitive value, if it doesn't exist then "i" will reach the value 9
	for i < 9 && len(sudoku[i][j]) == 1 {
		// If there is a signal of stop, kill the goroutine
		select {
		case <-stopChannel:
			stopChannel <- true
			return
		default:
			j++
			if j == 9 {
				i++
				j = 0
			}
		}
	}
	// If "i" has value 9 then the sudoku is already a solution, so it can be saved in the solution channel.
	// If there is already a solution in the channel, this upload will be ignored by the receiver
	if i == 9 {
		solutionChannel <- sudoku
	} else {
		// For every possible value of the found cell, copy the matrix, fix that value and try to find a solution
		for z := 0; z < len(sudoku[i][j]); z++ {
			// If there is a signal of stop, kill the goroutine
			select {
			case <-stopChannel:
				stopChannel <- true
				return
			default:
				// Copy the sudoku matrix
				sudokuCopy := copySudokuMatrix(sudoku)
				// Fix the value
				sudokuCopy[i][j] = []int{sudoku[i][j][z]}
				// Delete the contraints, in order to converge faster to a solution
				sudokuConstraintsElimination(sudokuCopy)
				// Before starting the new goroutine, check if the obtained matrix is correct
				if checkBaseSudokuCorrectness(sudokuCopy) {
					// Start a new goroutine that executes searchSolution with the new matrix
					go searchSolution(sudokuCopy, solutionChannel, stopChannel)
				}
			}
		}
	}
}

// Copy the sudoku matrix, copying every integer value
func copySudokuMatrix(sudoku [9][9][]int) [9][9][]int {
	var sudokuCopy [9][9][]int
	for i := 0; i < 9; i++ {
		for j := 0; j < 9; j++ {
			for z := 0; z < len(sudoku[i][j]); z++ {
				sudokuCopy[i][j] = append(sudokuCopy[i][j], sudoku[i][j][z])
			}
		}
	}
	return sudokuCopy
}

func countPossibilities(sudoku [9][9][]int) *big.Int {
	total := big.NewInt(1)
	for i := 0; i < 9; i++ {
		for j := 0; j < 9; j++ {
			total.Mul(total, big.NewInt(int64(len(sudoku[i][j]))))
		}
	}
	return total
}
