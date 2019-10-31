use std::sync::{Arc, RwLock};
mod SudokuIOManager;
use SudokuIOManager::sudokuIOManager;
mod SudokuManager;
use SudokuManager::sudoku;
mod CheckSudokuMethods;
use CheckSudokuMethods::{checkSudokuIsComplete, checkSudokuCorrectness};
mod BruteForceMethods;
use BruteForceMethods::bruteForceSolving;
mod ConstraintsElimination;
use ConstraintsElimination::constraintsElimination;
mod NakedPairs;
use NakedPairs::findNakedPairs;
mod NakedTriples;
use NakedTriples::findNakedTriples;

fn main() {
    let testBruteForce = false;

    if testBruteForce {
        // Load sudoku from file
        let path = "../../Sudokus/Brute Force Test.txt";
        let sudokuVar = sudoku::new(path.to_string());
        sudokuVar.printSudoku();
        println!("\n\n");

        // Run IOManager
        let ioManager = sudokuIOManager::new(sudokuVar);
        let ioManagerPointer = Arc::new(RwLock::new(ioManager));
        
        // Use brute force to find a solution if the sudoku is not complete. Note that it should be a method to avoid, because the
		// complexity is exponential and so a lot of constraints implies that a lot of time is required, although it uses parallelism to speed up
        SudokuIOManager::Run(ioManagerPointer.clone());

        {
            let newManager = bruteForceSolving(ioManagerPointer.clone());

            let newManagerPointer = Arc::new(RwLock::new(newManager));
            // Run the manager to delete the values sent in the specific channel
            SudokuIOManager::Run(newManagerPointer.clone());

            {
                let newManager = newManagerPointer.read().unwrap();
                newManager.PrintSudoku();
                if checkSudokuIsComplete(&newManager) {
                    println!("Sudoku is complete");
                }
                if checkSudokuCorrectness(&newManager) {
                    println!("The actual solution of the sudoku is correct");
                }
            }
        }
    } else {
        // Load sudoku from file
        let path = "../../Sudokus/Gentle.txt";
        let sudokuVar = sudoku::new(path.to_string());
        sudokuVar.printSudoku();
        println!("\n\n");

        // Run IOManager
        let ioManager = sudokuIOManager::new(sudokuVar);
        let ioManagerPointer = Arc::new(RwLock::new(ioManager));

        SudokuIOManager::Run(ioManagerPointer.clone());

        // Delete the invalid constraints
        constraintsElimination(ioManagerPointer.clone());
        {
            let ioManager = ioManagerPointer.read().unwrap();
            ioManager.PrintSudoku();
            println!{"\n\n"};
        }

        findNakedPairs(ioManagerPointer.clone());
        {
            let ioManager = ioManagerPointer.read().unwrap();
            ioManager.PrintSudoku();
            println!{"\n\n"};
        }

        findNakedTriples(ioManagerPointer.clone());
        {
            let ioManager = ioManagerPointer.read().unwrap();
            ioManager.PrintSudoku();
            println!{"\n\n"};
        }

        {
            let ioManager = ioManagerPointer.read().unwrap();
            if !checkSudokuIsComplete(&ioManager) {
                // Delete the lock to allow the contraint elimination in bruteForceSolving function
                drop(ioManager);
                let newManager = bruteForceSolving(ioManagerPointer.clone());
                let newManagerPointer = Arc::new(RwLock::new(newManager));
                // Run the manager to delete the values sent in the specific channel
                SudokuIOManager::Run(newManagerPointer.clone());

                {
                    let newManager = newManagerPointer.read().unwrap();
                    if checkSudokuIsComplete(&newManager) {
                        println!("Sudoku is complete");
                    }
                    if checkSudokuCorrectness(&newManager) {
                        println!("The actual solution of the sudoku is correct");
                    }
                }
            } else {
                println!("Sudoku is complete");
                if checkSudokuCorrectness(&ioManager) {
                    println!("The actual solution of the sudoku is correct");
                }
            }
        }
    }
}
