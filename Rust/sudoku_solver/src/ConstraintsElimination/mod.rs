extern crate crossbeam;
use crossbeam::crossbeam_channel::{Sender, bounded};
use std::sync::{Arc, RwLock, Mutex};
use std::time::SystemTime;
use super::SudokuIOManager::{sudokuIOManager, Run};
use super::SudokuManager::sudoku;

// Create a sudokuIOManager and call the function below
pub fn sudokuConstraintsElimination(sudoku: [[Vec<i8>; 9]; 9]) -> [[Vec<i8>; 9]; 9] {
    let sudokuContainer = sudoku::newContainer(sudoku);
    let ioManager = Arc::new(RwLock::new(sudokuIOManager::new(sudokuContainer)));
    Run(ioManager.clone());
    constraintsElimination(ioManager.clone());
    return ioManager.read().unwrap().GetSudoku();
}

// Delete all the horizontal, vertical and boxes invalid constraints
pub fn constraintsElimination(ioManager: Arc<RwLock<sudokuIOManager>>) {
    let start = SystemTime::now();
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
            // Start constraint elimination threads
            let manager = ioManager.clone();
            let channelSenderClone = horizontalChannelSender.clone();
            scope.spawn(move |_var| horizontalConstraintElimination(manager, channelSenderClone));
            let manager = ioManager.clone();
            let channelSenderClone = verticalChannelSender.clone();
            scope.spawn(move |_var| verticalConstraintElimination(manager, channelSenderClone));
            let manager = ioManager.clone();
            let channelSenderClone = boxesChannelSender.clone();
            scope.spawn(move |_var| boxesConstraintElimination(manager, channelSenderClone));
        });

        // Save the response of every channel in the respective boolean variable
        horizontalUpdates = horizontalChannelReceiver.recv().unwrap();
        verticalUpdates = verticalChannelReceiver.recv().unwrap();
        boxesUpdates = boxesChannelReceiver.recv().unwrap();
    }
    println!("ConstraintsElimination time: {}", (SystemTime::now().duration_since(start).expect("Time")).as_micros());
}

// Delete horizontal constraints, updates signals if there are deleted values
pub fn horizontalConstraintElimination(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    for i in 0..9 {
        for j in 0..9 {
            // Get the slice of the analyzed cell
            let readManager = ioManager.read().unwrap();
            let cellSlice = readManager.GetSlice(i, j);
            // Release the lock
            drop(readManager);
            // If the slice has a definitive value then delete the occurences of the same value in the row
            if cellSlice.len() == 1 {
                let cellValue = cellSlice[0];
                // Delete the value for the previous cells without definitive values
                for z in 0..j {
                    let readManager = ioManager.read().unwrap();
                    if readManager.GetSlice(i, z).len() > 1 && readManager.CheckNumber(i, z, cellValue) {
                        readManager.DeleteNumber(i, z, cellValue);
                        updates = true;
                    }
                }
                // Delete the value for the next cells without definitive values
                for z in j+1..9 {
                    let readManager = ioManager.read().unwrap();
                    if readManager.GetSlice(i, z).len() > 1 && readManager.CheckNumber(i, z, cellValue) {
                        readManager.DeleteNumber(i, z, cellValue);
                        updates = true;
                    }
                }
            }
        }
    }
    updatesChannel.lock().unwrap().send(updates);
}

// Delete vertical constraints, updates signals if there are deleted values
pub fn verticalConstraintElimination(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    for j in 0..9 {
        for i in 0..9 {
            // Get the slice of the analyzed cell
            let readManager = ioManager.read().unwrap();
            let cellSlice = readManager.GetSlice(i, j);
            // Release the lock
            drop(readManager);
            // If the slice has a definitive value then delete the occurences of the same value in the row
            if cellSlice.len() == 1 {
                let cellValue = cellSlice[0];
                // Delete the value for the previous cells without definitive values
                for z in 0..i {
                    let readManager = ioManager.read().unwrap();
                    if readManager.GetSlice(z, j).len() > 1 && readManager.CheckNumber(z, j, cellValue) {
                        readManager.DeleteNumber(z, j, cellValue);
                        updates = true;
                    }
                }
                // Delete the value for the next cells without definitive values
                for z in i+1..9 {
                    let readManager = ioManager.read().unwrap();
                    if readManager.GetSlice(z, j).len() > 1 && readManager.CheckNumber(z, j, cellValue) {
                        readManager.DeleteNumber(z, j, cellValue);
                        updates = true;
                    }
                }
            }
        }
    }
    updatesChannel.lock().unwrap().send(updates);
}

// Delete box constraints, updates signals if there are deleted values
pub fn boxesConstraintElimination(ioManager: Arc<RwLock<sudokuIOManager>>, updatesChannel: Arc<Mutex<Sender<bool>>>) {
    let mut updates = false;
    for i in 0..9 {
        for j in 0..9 {
            // Get the slice of the analyzed cell
            let readManager = ioManager.read().unwrap();
            let cellSlice = readManager.GetSlice(i, j);
            // Release the lock
            drop(readManager);
            // If the slice has a definitive value then delete the occurences of the same value in the box
            if cellSlice.len() == 1 {
                let cellValue = cellSlice[0];
                // Get the position of the analyzed box among the other boxes
                let boxRowPosition: i8 = i/3;
                let boxColumnPosition: i8 = j/3;
                // Update the cells of the box
                for ib in boxRowPosition*3..boxRowPosition*3+3 {
                    for jb in boxColumnPosition*3..boxColumnPosition*3+3 {
                        // Delete the value if the cell doesn't have a definitive value (note that the
						// original cell is excluded automatically because it has a definitive value)
                        let readManager = ioManager.read().unwrap();
                        if readManager.GetSlice(ib, jb).len() > 1 && readManager.CheckNumber(ib, jb, cellValue) {
                            readManager.DeleteNumber(ib, jb, cellValue);
                            updates = true;
                        }
                    }
                }
            }
        }
    }
    updatesChannel.lock().unwrap().send(updates);
}