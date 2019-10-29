extern crate crossbeam;
use crossbeam::crossbeam_channel::{Sender, bounded};
use super::SudokuManager::{sudoku, contains};
use super::SudokuIOManager::sudokuIOManager;

// Check if there are cells without definitive values
pub fn checkSudokuIsComplete(ioManager: &sudokuIOManager) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if ioManager.GetSlice(i, j).len() > 1 {
                return false;
            }
        }
    }
    return true;
}

// Create a sudokuIOManager and call the function below
pub fn checkBaseSudokuCorrectness(sudoku: [[Vec<i8>; 9]; 9]) -> bool {
    let sudokuContainer = sudoku::newContainer(sudoku);
    let ioManager = sudokuIOManager::new(sudokuContainer);
    return checkSudokuCorrectness(&ioManager);
}

// Check if the cells with definitive values have valid numbers
pub fn checkSudokuCorrectness(ioManager: &sudokuIOManager) -> bool {
    // Make channels to communicate the correctness
    let (horizontalChannelSender, horizontalChannelReceiver)  = bounded(1);
    let (verticalChannelSender, verticalChannelReceiver)  = bounded(1);
    let (boxesChannelSender, boxesChannelReceiver)  = bounded(1);

    crossbeam::scope(|scope| {
        // Start the subroutines to check horizontal, vertical and boxes correctness
        scope.spawn(move |_var| checkHorizontalCorrectness(ioManager, horizontalChannelSender));
        scope.spawn(move |_var| checkVerticalCorrectness(ioManager, verticalChannelSender));
        scope.spawn(move |_var| checkBoxesCorrectness(ioManager, boxesChannelSender));
    });

    // Return the result of the three analysis
    return horizontalChannelReceiver.recv().unwrap() && verticalChannelReceiver.recv().unwrap() && boxesChannelReceiver.recv().unwrap();
}

// Check if all rows are correct
fn checkHorizontalCorrectness(ioManager: &sudokuIOManager, updatesChannel: Sender<bool>) {
    let mut correct = true;
    let mut i = 0;
    while i < 9 && correct {
        // List of already analysed values
        let mut valuesList: Vec<i8> = Default::default();
        let mut j = 0;
        while j < 9 && correct {
            let cellSlice = ioManager.GetSlice(i, j);
            // Only cells with definitive values are analysed
            if cellSlice.len() == 1 {
                if contains(&valuesList, cellSlice[0]) {
                    correct = false;
                } else {
                    // Save the value for next checks
                    valuesList.push(cellSlice[0]);
                }
            }
            j += 1;
        }
        i += 1;
    }
    updatesChannel.send(correct).unwrap();
}

// Check if all columns are correct
fn checkVerticalCorrectness(ioManager: &sudokuIOManager, updatesChannel: Sender<bool>) {
    let mut correct = true;
    let mut j = 0;
    while j < 9 && correct {
        // List of already analysed values
        let mut valuesList: Vec<i8> = Default::default();
        let mut i = 0;
        while i < 9 && correct {
            let cellSlice = ioManager.GetSlice(i, j);
            // Only cells with definitive values are analysed
            if cellSlice.len() == 1 {
                if contains(&valuesList, cellSlice[0]) {
                    correct = false;
                } else {
                    // Save the value for next checks
                    valuesList.push(cellSlice[0]);
                }
            }
            i += 1;
        }
        j += 1;
    }
    updatesChannel.send(correct).unwrap();
}

// Check if all boxes are correct
fn checkBoxesCorrectness(ioManager: &sudokuIOManager, updatesChannel: Sender<bool>) {
    let mut correct = true;
    for i in 0..3 {
        for j in 0..3 {
            // List of already analysed values
            let mut valuesList: Vec<i8> = Default::default();
            for ib in 0..3 {
                for jb in 0..3 {
                    let cellSlice = ioManager.GetSlice(i*3+ib, j*3+jb);
                    if cellSlice.len() == 1 {
                        if contains(&valuesList, cellSlice[0]) {
                            correct = false;
                        } else {
                            // Save the value for next checks
                            valuesList.push(cellSlice[0]);
                        }
                    }
                }
            }
        }
    }
    updatesChannel.send(correct).unwrap();
}