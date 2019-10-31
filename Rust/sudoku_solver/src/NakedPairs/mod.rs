extern crate crossbeam;
use crossbeam::crossbeam_channel::{Sender, bounded};
use std::sync::{Arc, RwLock, Mutex};
use super::SudokuIOManager::sudokuIOManager;
use super::SudokuManager::contains;
use super::ConstraintsElimination::constraintsElimination;

// Find all the horizontal, vertical and boxes naked pairs
pub fn findNakedPairs(ioManager: Arc<RwLock<sudokuIOManager>>) {
    // Booleans to check if there are deleted values in the following threads
    let mut horizontalUpdates = true;
    let mut verticalUpdates = true;
    let mut boxesUpdates = true;

    // Channels used by the threads to communicate that there is at least one value modified
    let (horizontalChannelSender, horizontalChannelReceiver)  = bounded(1);
    let (verticalChannelSender, verticalChannelReceiver)  = bounded(1);
    let (boxesChannelSender, boxesChannelReceiver)  = bounded(1);

    let horizontalChannelSender = Arc::new(Mutex::new(horizontalChannelSender));
    let verticalChannelSender = Arc::new(Mutex::new(verticalChannelSender));
    let boxesChannelSender = Arc::new(Mutex::new(boxesChannelSender));

    // While there is at least one thread that updates one or more values, the three threads must be executed again
    while horizontalUpdates || verticalUpdates || boxesUpdates {
        crossbeam::scope(|scope| {
            // Start finding naked pairs threads
            let manager = ioManager.clone();
            let channelSenderClone = horizontalChannelSender.clone();
            scope.spawn(move |_var| findHorizontalNakedPairs(manager, channelSenderClone));
            let manager = ioManager.clone();
            let channelSenderClone = verticalChannelSender.clone();
            scope.spawn(move |_var| findVerticalNakedPairs(manager, channelSenderClone));
            let manager = ioManager.clone();
            let channelSenderClone = boxesChannelSender.clone();
            scope.spawn(move |_var| findBoxesNakedPairs(manager, channelSenderClone));
        });

        // Save the response of every channel in the respective boolean variable
        horizontalUpdates = horizontalChannelReceiver.recv().unwrap();
        verticalUpdates = verticalChannelReceiver.recv().unwrap();
        boxesUpdates = boxesChannelReceiver.recv().unwrap();

        if horizontalUpdates || verticalUpdates || boxesUpdates {
            // If there are deleted values then the constraints must be updated
            constraintsElimination(ioManager.clone());
        }
    }
}

// Find the couples of cells in the same row with two identical possible numbers
pub fn findHorizontalNakedPairs(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    let readManager = ioManager.read().unwrap();
    let sudokuCopy = readManager.GetSudoku();
    drop(readManager);
    for i in 0..9 {
        // Save the index of every cell with two elements, in order to minimize the number of analyzed cells
        let mut validCellsPosition = Vec::new();
        let mut validCombination = false;
        for j in 0..9 {
            if sudokuCopy[i][j].len() == 2 {
                validCellsPosition.push(j as i8);
            }
        }
        // If there are zero or one cells with two possible numbers, there can't be naked pairs in this row
        if validCellsPosition.len() > 1 {
            // Find the combinations of the cells
            let combinationsList = getCombinations(validCellsPosition, 2, 0);
            // Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
            let mut z = 0;
            while z < combinationsList.len() && !validCombination {
                // Save the indexes of the analyzed cells columns in specific variables for convenience
                let firstValuePosition = combinationsList[z][0] as usize;
                let secondValuePosition = combinationsList[z][1] as usize;
                // Check if the two possible values of the first cell of the combination are the same of the second cell
                if contains(&sudokuCopy[i][secondValuePosition], sudokuCopy[i][firstValuePosition][0]) &&
                    contains(&sudokuCopy[i][secondValuePosition], sudokuCopy[i][firstValuePosition][1]) {
                        // This is a naked pair, so remove these two values from the other cells of the row
                        for j in 0..9 {
                            // If the analyzed cell is not one of the combination cells delete the two possible numbers if they belongs to its possible values
                            if j != firstValuePosition && j != secondValuePosition &&
                                (checkSliceElement(&sudokuCopy[i][j], sudokuCopy[i][firstValuePosition][0]) || checkSliceElement(&sudokuCopy[i][j], sudokuCopy[i][firstValuePosition][1])) {
                                    let readManager = ioManager.read().unwrap();
                                    readManager.DeleteNumber(i as i8, j as i8, sudokuCopy[i][firstValuePosition][0]);
                                    readManager.DeleteNumber(i as i8, j as i8, sudokuCopy[i][firstValuePosition][1]);
                                    updates = true;
                                    validCombination = true;
                            }
                        }
                }
                z += 1;
            }
        }
    }
    updatesChannel.lock().unwrap().send(updates);
}

// Find the couples of cells in the same column with two identical possible numbers
pub fn findVerticalNakedPairs(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    let readManager = ioManager.read().unwrap();
    let sudokuCopy = readManager.GetSudoku();
    drop(readManager);
    for j in 0..9 {
        // Save the index of every cell with two elements, in order to minimize the number of analyzed cells
        let mut validCellsPosition = Vec::new();
        let mut validCombination = false;
        for i in 0..9 {
            if sudokuCopy[i][j].len() == 2 {
                validCellsPosition.push(i as i8);
            }
        }
        // If there are zero or one cells with two possible numbers, there can't be naked pairs in this column
        if validCellsPosition.len() > 1 {
            // Find the combinations of the cells
            let combinationsList = getCombinations(validCellsPosition, 2, 0);
            // Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
            let mut z = 0;
            while z < combinationsList.len() && !validCombination {
                // Save the indexes of the analyzed cells rows in specific variables for convenience
                let firstValuePosition = combinationsList[z][0] as usize;
                let secondValuePosition = combinationsList[z][1] as usize;
                // Check if the two possible values of the first cell of the combination are the same of the second cell
                if contains(&sudokuCopy[secondValuePosition][j], sudokuCopy[firstValuePosition][j][0]) &&
                    contains(&sudokuCopy[secondValuePosition][j], sudokuCopy[firstValuePosition][j][1]) {
                        // This is a naked pair, so remove these two values from the other cells of the row
                        for i in 0..9 {
                            // If the analyzed cell is not one of the combination cells delete the two possible numbers if they belongs to its possible values
                            if i != firstValuePosition && i != secondValuePosition &&
                                (checkSliceElement(&sudokuCopy[i][j], sudokuCopy[firstValuePosition][j][0]) || checkSliceElement(&sudokuCopy[i][j], sudokuCopy[firstValuePosition][j][1])) {
                                    let readManager = ioManager.read().unwrap();
                                    readManager.DeleteNumber(i as i8, j as i8, sudokuCopy[firstValuePosition][j][0]);
                                    readManager.DeleteNumber(i as i8, j as i8, sudokuCopy[firstValuePosition][j][1]);
                                    updates = true;
                                    validCombination = true;
                            }
                        }
                }
                z += 1;
            }
        }
    }
    updatesChannel.lock().unwrap().send(updates);
}

// Find the couples of cells in the same box with two identical possible numbers
pub fn findBoxesNakedPairs(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    let readManager = ioManager.read().unwrap();
    let sudokuCopy = readManager.GetSudoku();
    drop(readManager);
    for i in 0..3 {
        for j in 0..3 {
            // Save the index of every cell with two elements, in order to minimize the number of analyzed cells
            let mut validCellsPosition = Vec::new();
            let mut validCombination = false;
            for ib in 0..3 {
                for jb in 0..3 {
                    if sudokuCopy[i*3+ib][j*3+jb].len() == 2 {
                        // Save the position with this formula to find the row an the column cell using a single number
                        validCellsPosition.push((ib*3+jb) as i8);
                    }
                }
            }
            // If there are zero or one cells with two possible numbers, there can't be naked pairs in this box
            if validCellsPosition.len() > 1 {
                // Find the combinations of the cells
                let combinationsList = getCombinations(validCellsPosition, 2, 0);
                // Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
                let mut z = 0;
                while z < combinationsList.len() && !validCombination {
                    // Save the indexes of the analyzed cells boxes in specific variables for convenience
                    let firstValueRowPosition = combinationsList[z][0] as usize / 3;
                    let firstValueColumnPosition = combinationsList[z][0] as usize % 3;
                    let secondValueRowPosition = combinationsList[z][1] as usize / 3;
                    let secondValueColumnPosition = combinationsList[z][1] as usize % 3;
                    // Check if the two possible values of the first cell of the combination are the same of the second cell
                    if contains(&sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][0]) &&
                        contains(&sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][1]) {
                        // This is a naked pair, so remove these two values from the other cells of the box
                        for ib in 0..3 {
                            for jb in 0..3 {
                                // If the analyzed cell is not one of the combination cells delete the two possible numbers (if there isn't a number the requests will be ignored)
                                if (ib != firstValueRowPosition || jb != firstValueColumnPosition) && (ib != secondValueRowPosition || jb != secondValueColumnPosition) &&
                                    (checkSliceElement(&sudokuCopy[i*3+ib][j*3+jb], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][0]) || checkSliceElement(&sudokuCopy[i*3+ib][j*3+jb], sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][1])) {

                                    let readManager = ioManager.read().unwrap();
                                    readManager.DeleteNumber((i*3+ib) as i8, (j*3+jb) as i8, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][0]);
                                    readManager.DeleteNumber((i*3+ib) as i8, (j*3+jb) as i8, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][1]);
                                    updates = true;
                                    validCombination = true;
                                }
                            }
                        }
                    }
                    z += 1;
                }
            }
        }
    }
    updatesChannel.lock().unwrap().send(updates);
}

// Utility functions for combinations
pub fn getCombinations(arr: Vec<i8>, leng: i8, startPosition: i8) -> Vec<Vec<i8>> {
    let mut result: Vec<i8> = Vec::new();
    for _i in 0..leng {
        result.push(0);
    }
    return combinations(&arr, leng, startPosition, &mut result);
}

// Return all possible combinations of length leng of the elements in arr
fn combinations(arr: &Vec<i8>, leng: i8, startPosition: i8, result: &mut Vec<i8>) -> Vec<Vec<i8>> {
    let mut finalResult = Vec::new();
    if leng == 0 {
        // In result there is one combination, which copied and saved in finalResult
        finalResult.push(result.to_vec());
        return finalResult;
    }
    //let numb = arr.len() as i8 - leng + 1;
    for i in startPosition..(arr.len() as i8 - leng + 1) {
        let resleng = result.len();
        result[resleng - leng as usize] = arr[i as usize];
        // Append to finalResult the combinations obtained with the recursive calls of this function
        finalResult.append(&mut combinations(&arr, leng-1, i+1, result));
    }
    return finalResult;
}

pub fn checkSliceElement(slice: &Vec<i8>, value: i8) -> bool {
    return slice.len() > 1 && contains(&slice, value);
}