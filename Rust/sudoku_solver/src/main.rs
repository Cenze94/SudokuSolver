use std::thread;
mod SudokuIOManager;
use SudokuIOManager::sudokuIOManager;
mod SudokuManager;
use SudokuManager::sudoku;

fn main() {
    let testBruteForce = false;

    if testBruteForce {
        // Load sudoku from file
        let path = "../../Sudokus/Brute Force Test.txt";
        let sudokuVar = sudoku::new(path.to_string());
        sudokuVar.printSudoku();
        println!("\n\n");

        // Run IOManager
        let mut ioManager = sudokuIOManager::new(sudokuVar);
        thread::spawn(move || {
            ioManager.Run();
        });


    } else {
        // Load sudoku from file
        let path = "../../Sudokus/Gentle.txt";
        let sudokuVar = sudoku::new(path.to_string());
        sudokuVar.printSudoku();
        println!("\n\n");

        // Run IOManager
        let mut ioManager = sudokuIOManager::new(sudokuVar);
        thread::spawn(move || {
            ioManager.Run();
        });


    }
}
