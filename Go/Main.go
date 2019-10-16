package main

import (
	"fmt"
)

func main() {
	testBruteForce := false

	if testBruteForce {
		// Load sudoku from file
		path := "../Sudokus/Brute Force Test.txt"
		var sudokuVar sudoku = new(path)
		sudokuVar.printSudoku()
		fmt.Println("\n\n")

		// Run IOManager
		var ioManager sudokuIOManager = sudokuIOManager{&sudokuVar, make(chan sudokuDeleteNumber), make(chan bool), make(chan [9][9][]int)}
		go ioManager.Run()

		// Use brute force to find a solution if the sudoku is not complete. Note that it should be a method to avoid, because the
		// complexity is exponential and so a lot of constraints implies that a lot of time is required, although it uses parallelism to speed up
		sudokuSolutionManager := bruteForceSolving(ioManager)
		sudokuSolutionManager.PrintSudoku()

		if checkSudokuIsComplete(sudokuSolutionManager) {
			fmt.Println("Sudoku is complete")
		}
		if checkSudokuCorrectness(sudokuSolutionManager) {
			fmt.Println("The actual solution of the sudoku is correct")
		}
	} else {
		// Load sudoku from file
		path := "../Sudokus/Gentle.txt"
		var sudokuVar sudoku = new(path)
		sudokuVar.printSudoku()
		fmt.Println("\n\n")

		// Run IOManager
		var ioManager sudokuIOManager = sudokuIOManager{&sudokuVar, make(chan sudokuDeleteNumber), make(chan bool), make(chan [9][9][]int)}
		go ioManager.Run()

		// Delete the invalid constraints
		constraintsElimination(ioManager)
		ioManager.PrintSudoku()
		fmt.Println("\n\n")

		findNakedPairs(ioManager)
		ioManager.PrintSudoku()
		fmt.Println("\n\n")

		findNakedTriples(ioManager)
		ioManager.PrintSudoku()
		fmt.Println("\n\n")

		// Use brute force to find a solution if the sudoku is not complete. Note that it should be a method to avoid, because the
		// complexity is exponential and so a lot of constraints implies that a lot of time is required, although it uses parallelism to speed up
		if !checkSudokuIsComplete(ioManager) {
			sudokuSolutionManager := bruteForceSolving(ioManager)
			sudokuSolutionManager.PrintSudoku()

			if checkSudokuIsComplete(sudokuSolutionManager) {
				fmt.Println("Sudoku is complete")
			}
			if checkSudokuCorrectness(sudokuSolutionManager) {
				fmt.Println("The actual solution of the sudoku is correct")
			}
		} else {
			fmt.Println("Sudoku is complete")
			if checkSudokuCorrectness(ioManager) {
				fmt.Println("The actual solution of the sudoku is correct")
			}
		}
	}
}
