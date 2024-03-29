extern crate crossbeam;
use crossbeam::crossbeam_channel::{Receiver, Sender, Select, unbounded};
use super::SudokuManager::sudoku;
use std::sync::{Arc, RwLock};
use std::thread;

// Contain the variables needed to delete a specific value from a given cell
struct sudokuDeleteNumber {
    row:i8, column:i8, value:i8
}

// Manager of sudoku output channels
pub struct sudokuIOManager {
    pub sudokuVar: sudoku,
    deleteSender: Sender<sudokuDeleteNumber>,
    deleteReceiver: Receiver<sudokuDeleteNumber>,
    pub requestSudokuSender: Sender<bool>,
    pub requestSudokuReceiver: Receiver<bool>,
    pub sendSudokuSender: Sender<[[Vec<i8>; 9]; 9]>,
    pub sendSudokuReceiver: Receiver<[[Vec<i8>; 9]; 9]>
}

impl sudokuIOManager {
    pub fn new(sudokuVar: sudoku) -> sudokuIOManager {
        let (deleteSender, deleteReceiver) = unbounded();
        let (requestSudokuSender, requestSudokuReceiver) = unbounded();
        let (sendSudokuSender, sendSudokuReceiver) = unbounded();
        return sudokuIOManager{sudokuVar, deleteSender, deleteReceiver,
            requestSudokuSender, requestSudokuReceiver, sendSudokuSender, sendSudokuReceiver
        }
    }

    // This method adds the data about the number to delete to the delete channel
    pub fn DeleteNumber(&self, i:i8, j:i8, value:i8) {
        self.deleteSender.send(sudokuDeleteNumber{row: i, column: j, value: value}).unwrap();
    }

    // This method returns the slice of the cell in the position given in input.
    // The consequence is that every thread will work with the IOManager instead of the sudoku
    pub fn GetSlice(&self, i:i8, j:i8) -> Vec<i8> {
        return self.sudokuVar.getCellValue(i, j);
    }

    // This method check if the given value is contained in the cell in position i and j
    pub fn CheckNumber(&self, i:i8, j:i8, value:i8) -> bool {
        return self.sudokuVar.checkCellValue(i, j, value);
    }

    // Return the sudoku for the brute force methods
    pub fn GetSudoku(&self) -> [[Vec<i8>; 9]; 9] {
        self.requestSudokuSender.send(true).unwrap();
        return self.sendSudokuReceiver.recv().unwrap();
    }

    pub fn PrintSudoku(&self) {
        self.sudokuVar.printSudoku();
    }
}

// Run method of sudokuIOManager, for every value to delete check if the number exists (because of the
// execution of multiple concurrent threads)
pub fn Run(ioManagerPointer: Arc<RwLock<sudokuIOManager>>) {
    thread::spawn(move || {
        loop {
            let ioManager = ioManagerPointer.read().unwrap();
            let srRes = ioManager.requestSudokuReceiver.try_recv();
            // Check if there is a request of a copy of the sudoku
            if srRes.is_ok() {
                let res = srRes.unwrap();

                // Create and send the sudoku copy
                let mut matrixCopy: [[Vec<i8>; 9]; 9] = Default::default();
                for i in 0..9 {
                    for j in 0..9 {
                        for z in 0..ioManager.sudokuVar.sudokuMatrix[i][j].len() {
                            matrixCopy[i][j].push(ioManager.sudokuVar.sudokuMatrix[i][j][z]);
                        }
                    }
                }
                ioManager.sendSudokuSender.send(matrixCopy).unwrap();
            } else {
                // Release the read lock and try to take the write one
                drop(ioManager);
                let result = ioManagerPointer.try_write();
                if result.is_ok() {
                    let mut ioManager = result.unwrap();
                    // Check if there is an element to delete
                    let recRes = ioManager.deleteReceiver.try_recv();
                    if recRes.is_ok() {
                        let res = recRes.unwrap();
                        // Delete the element
                        if ioManager.sudokuVar.checkCellValue(res.row, res.column, res.value) {
                            ioManager.sudokuVar.deleteCellValue(res.row, res.column, res.value);
                        }
                    }
                }
            }
        }
    });
}