extern crate crossbeam;
use crossbeam::crossbeam_channel::{Sender, bounded};
use std::sync::{Arc, RwLock, Mutex};
use super::SudokuIOManager::sudokuIOManager;
use super::SudokuManager::contains;
use super::ConstraintsElimination::constraintsElimination;
use super::NakedPairs::{findNakedPairs, getCombinations, checkSliceElement};

// Find all the horizontal, vertical and boxes naked triples
pub fn findNakedTriples(ioManager: Arc<RwLock<sudokuIOManager>>) {
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
            // Start finding naked triples threads
            let manager = ioManager.clone();
            let channelSenderClone = horizontalChannelSender.clone();
            scope.spawn(move |_var| findHorizontalNakedTriples(manager, channelSenderClone));
            let manager = ioManager.clone();
            let channelSenderClone = verticalChannelSender.clone();
            scope.spawn(move |_var| findVerticalNakedTriples(manager, channelSenderClone));
            let manager = ioManager.clone();
            let channelSenderClone = boxesChannelSender.clone();
            scope.spawn(move |_var| findBoxesNakedTriples(manager, channelSenderClone));
        });

        // Save the response of every channel in the respective boolean variable
        horizontalUpdates = horizontalChannelReceiver.recv().unwrap();
        verticalUpdates = verticalChannelReceiver.recv().unwrap();
        boxesUpdates = boxesChannelReceiver.recv().unwrap();

        if horizontalUpdates || verticalUpdates || boxesUpdates {
            // If there are deleted values then the constraints must be updated
            constraintsElimination(ioManager.clone());
            // After the elimination of the values there could be naked pairs
            findNakedPairs(ioManager.clone());
        }
    }
}

// Find the triples of cells in the same row with two or three identical possible numbers
pub fn findHorizontalNakedTriples(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    let readManager = ioManager.read().unwrap();
    let sudokuCopy = readManager.GetSudoku();
    drop(readManager);
    for i in 0..9 {
        // Save the index of every cell with two or three elements, in order to minimize the number of analyzed cells
        let mut validCellsPosition = Vec::new();
        let mut validCombination = false;
        for j in 0..9 {
            if sudokuCopy[i][j].len() == 2 || sudokuCopy[i][j].len() == 3 {
                validCellsPosition.push(j as i8);
            }
        }
        // If there are less than three cells with two or three possible numbers, there can't be naked triples in this row
        if validCellsPosition.len() > 2 {
            // Find the combinations of the cells
            let combinationsList = getCombinations(validCellsPosition, 3, 0);
            // Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
            let mut z = 0;
            while z < combinationsList.len() && !validCombination {
                // Save the indexes of the analyzed cells columns in specific variables for convenience
                let firstValuePosition = combinationsList[z][0] as usize;
                let secondValuePosition = combinationsList[z][1] as usize;
                let thirdValuePosition = combinationsList[z][2] as usize;
                // Get the union of the possible values of the analyzed cells
                let mut values = Vec::new();
                for t in 0..2 {
                    if !contains(&values, sudokuCopy[i][firstValuePosition][t]) {
                        values.push(sudokuCopy[i][firstValuePosition][t]);
                    }
                    if !contains(&values, sudokuCopy[i][secondValuePosition][t]) {
                        values.push(sudokuCopy[i][secondValuePosition][t]);
                    }
                    if !contains(&values, sudokuCopy[i][thirdValuePosition][t]) {
                        values.push(sudokuCopy[i][thirdValuePosition][t]);
                    }
                }
                // The analyzed cells could not have the third element
                let slice = &sudokuCopy[i][firstValuePosition];
                if slice.len() == 3 && !contains(&values, slice[2]) {
                    values.push(sudokuCopy[i][firstValuePosition][2]);
                }
                let slice = &sudokuCopy[i][secondValuePosition];
                if slice.len() == 3 && !contains(&values, slice[2]) {
                    values.push(sudokuCopy[i][secondValuePosition][2]);
                }
                let slice = &sudokuCopy[i][thirdValuePosition];
                if slice.len() == 3 && !contains(&values, slice[2]) {
                    values.push(sudokuCopy[i][thirdValuePosition][2]);
                }
                // If the size of the union is three then the analyzed cells are a naked triple
                if values.len() == 3 {
                    // Remove these values from the other cells of the row
                    for j in 0..9 {
                        if j != firstValuePosition && j != secondValuePosition && j != thirdValuePosition &&
                            (checkSliceElement(&sudokuCopy[i][j], values[0]) || checkSliceElement(&sudokuCopy[i][j], values[1]) || checkSliceElement(&sudokuCopy[i][j], values[2])) {

                            let readManager = ioManager.read().unwrap();
                            readManager.DeleteNumber(i as i8, j as i8, values[0]);
                            readManager.DeleteNumber(i as i8, j as i8, values[1]);
                            readManager.DeleteNumber(i as i8, j as i8, values[2]);
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

// Find the triples of cells in the same column with two or three identical possible numbers
pub fn findVerticalNakedTriples(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    let readManager = ioManager.read().unwrap();
    let sudokuCopy = readManager.GetSudoku();
    drop(readManager);
    for j in 0..9 {
        // Save the index of every cell with two or three elements, in order to minimize the number of analyzed cells
        let mut validCellsPosition = Vec::new();
        let mut validCombination = false;
        for i in 0..9 {
            if sudokuCopy[i][j].len() == 2 || sudokuCopy[i][j].len() == 3 {
                validCellsPosition.push(i as i8);
            }
        }
        // If there are less than three cells with two or three possible numbers, there can't be naked triples in this column
        if validCellsPosition.len() > 2 {
            // Find the combinations of the cells
            let combinationsList = getCombinations(validCellsPosition, 3, 0);
            // Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
            let mut z = 0;
            while z < combinationsList.len() && !validCombination {
                // Save the indexes of the analyzed cells rows in specific variables for convenience
                let firstValuePosition = combinationsList[z][0] as usize;
                let secondValuePosition = combinationsList[z][1] as usize;
                let thirdValuePosition = combinationsList[z][2] as usize;
                // Get the union of the possible values of the analyzed cells
                let mut values = Vec::new();
                for t in 0..2 {
                    if !contains(&values, sudokuCopy[firstValuePosition][j][t]) {
                        values.push(sudokuCopy[firstValuePosition][j][t]);
                    }
                    if !contains(&values, sudokuCopy[secondValuePosition][j][t]) {
                        values.push(sudokuCopy[secondValuePosition][j][t]);
                    }
                    if !contains(&values, sudokuCopy[thirdValuePosition][j][t]) {
                        values.push(sudokuCopy[thirdValuePosition][j][t]);
                    }
                }
                // The analyzed cells could not have the third element
                let slice = &sudokuCopy[firstValuePosition][j];
                if slice.len() == 3 && !contains(&values, slice[2]) {
                    values.push(sudokuCopy[firstValuePosition][j][2]);
                }
                let slice = &sudokuCopy[secondValuePosition][j];
                if slice.len() == 3 && !contains(&values, slice[2]) {
                    values.push(sudokuCopy[secondValuePosition][j][2]);
                }
                let slice = &sudokuCopy[thirdValuePosition][j];
                if slice.len() == 3 && !contains(&values, slice[2]) {
                    values.push(sudokuCopy[thirdValuePosition][j][2]);
                }
                // If the size of the union is three then the analyzed cells are a naked triple
                if values.len() == 3 {
                    // Remove these values from the other cells of the column
                    for i in 0..9 {
                        if i != firstValuePosition && i != secondValuePosition && i != thirdValuePosition &&
                            (checkSliceElement(&sudokuCopy[i][j], values[0]) || checkSliceElement(&sudokuCopy[i][j], values[1]) || checkSliceElement(&sudokuCopy[i][j], values[2])) {

                            let readManager = ioManager.read().unwrap();
                            readManager.DeleteNumber(i as i8, j as i8, values[0]);
                            readManager.DeleteNumber(i as i8, j as i8, values[1]);
                            readManager.DeleteNumber(i as i8, j as i8, values[2]);
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

// Find the triples of cells in the same box with two or three identical possible numbers
pub fn findBoxesNakedTriples(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    let readManager = ioManager.read().unwrap();
    let sudokuCopy = readManager.GetSudoku();
    drop(readManager);
    for i in 0..3 {
        for j in 0..3 {
            // Save the index of every cell with two or three elements, in order to minimize the number of analyzed cells
            let mut validCellsPosition = Vec::new();
            let mut validCombination = false;
            for ib in 0..3 {
                for jb in 0..3 {
                    if sudokuCopy[i*3+ib][j*3+jb].len() == 2 || sudokuCopy[i*3+ib][j*3+jb].len() == 3 {
                        // Save the position with this formula to find the row an the column cell using a single number
                        validCellsPosition.push((ib*3+jb) as i8);
                    }
                }
            }
            // If there are less than three cells with two or three possible numbers, there can't be naked triples in this box
            if validCellsPosition.len() > 2 {
                // Find the combinations of the cells
                let combinationsList = getCombinations(validCellsPosition, 3, 0);
                // Only the first valid combination is considered (with the check of validCombination variable) because the following will be invalid
                let mut z = 0;
                while z < combinationsList.len() && !validCombination {
                    // Save the indexes of the analyzed cells rows and columns in specific variables for convenience
                    let firstValueRowPosition = combinationsList[z][0] as usize / 3;
                    let firstValueColumnPosition = combinationsList[z][0] as usize % 3;
                    let secondValueRowPosition = combinationsList[z][1] as usize / 3;
                    let secondValueColumnPosition = combinationsList[z][1] as usize % 3;
                    let thirdValueRowPosition = combinationsList[z][2] as usize / 3;
                    let thirdValueColumnPosition = combinationsList[z][2] as usize % 3;
                    // Get the union of the possible values of the analyzed cells
                    let mut values = Vec::new();
                    for t in 0..2 {
                        if !contains(&values, sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][t]) {
                            values.push(sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][t]);
                        }
                        if !contains(&values, sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition][t]) {
                            values.push(sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition][t]);
                        }
                        if !contains(&values, sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition][t]) {
                            values.push(sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition][t]);
                        }
                    }
                    // The analyzed cells could not have the third element
                    let slice = &sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition];
                    if slice.len() == 3 && !contains(&values, slice[2]) {
                        values.push(sudokuCopy[i*3+firstValueRowPosition][j*3+firstValueColumnPosition][2]);
                    }
                    let slice = &sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition];
                    if slice.len() == 3 && !contains(&values, slice[2]) {
                        values.push(sudokuCopy[i*3+secondValueRowPosition][j*3+secondValueColumnPosition][2]);
                    }
                    let slice = &sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition];
                    if slice.len() == 3 && !contains(&values, slice[2]) {
                        values.push(sudokuCopy[i*3+thirdValueRowPosition][j*3+thirdValueColumnPosition][2]);
                    }
                    // If the size of the union is three then the analyzed cells are a naked triple
                    if values.len() == 3 {
                        // Remove these values from the other cells of the box
                        for ib in 0..3 {
                            for jb in 0..3 {
                                if (ib != firstValueRowPosition || jb != firstValueColumnPosition) && (ib != secondValueRowPosition || jb != secondValueColumnPosition) && (ib != thirdValueRowPosition || jb != thirdValueColumnPosition) &&
                                    (checkSliceElement(&sudokuCopy[i*3+ib][j*3+jb], values[0]) || checkSliceElement(&sudokuCopy[i*3+ib][j*3+jb], values[1]) || checkSliceElement(&sudokuCopy[i*3+ib][j*3+jb], values[2])) {

                                    let readManager = ioManager.read().unwrap();
                                    readManager.DeleteNumber((i*3+ib) as i8, (j*3+jb) as i8, values[0]);
                                    readManager.DeleteNumber((i*3+ib) as i8, (j*3+jb) as i8, values[1]);
                                    readManager.DeleteNumber((i*3+ib) as i8, (j*3+jb) as i8, values[2]);
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