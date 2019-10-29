extern crate crossbeam;
extern crate num_bigint;
extern crate num_traits;
use crossbeam::crossbeam_channel::{Sender, Receiver, unbounded};
use num_bigint::BigInt;
use num_traits::One;
use super::SudokuManager::{sudoku};
use super::SudokuIOManager::sudokuIOManager;
use super::CheckSudokuMethods;
use super::ConstraintsElimination::sudokuConstraintsElimination;
use std::sync::{Arc, RwLock, Mutex};

// Get the solution using brute force, creating a specific channel and calling "searchSolution" as a goroutine
pub fn bruteForceSolving(ioManagerPointer: Arc<RwLock<sudokuIOManager>>) -> sudokuIOManager {
    let (solutionChannelSender, solutionChannelReceiver) = unbounded();
    // Create a channel to send a stop signal to the active threads. The size is arbitrary, its purpose is to make the channel non-blocking
    let (stopChannelSender, stopChannelReceiver) = unbounded();
    let stopChannelReceiver = Arc::new(Mutex::new(stopChannelReceiver));
    let stopChannelSender = Arc::new(Mutex::new(stopChannelSender));
    {
        let ioManager = ioManagerPointer.read().unwrap();
        let sudokuTable = ioManager.GetSudoku();
        let stopChannelSenderClone = stopChannelSender.clone();
        crossbeam::scope(|scope| {
            scope.spawn(move |_var| {searchSolution(sudokuTable, &solutionChannelSender, stopChannelSenderClone, stopChannelReceiver.clone())});
        });
    }
    let sudokuSolution = solutionChannelReceiver.recv().unwrap();
    // Send a stop signal to the running threads, because there is already a correct solution.
    // If there isn't a running thread, an error will be returned because the channel receiver will be destroyed. Since this is not a problem, the case isn't handled.
    stopChannelSender.lock().unwrap().send(true);

    // Save and return the obtained solution in a new sudokuIOManager
    let sudokuContainer = sudoku::newContainer(sudokuSolution);
    return sudokuIOManager::new(sudokuContainer);
}

// Recursive function, called as a subroutine in order to parallelize the search
pub fn searchSolution(sudoku: [[Vec<i8>; 9]; 9], solutionChannel: &Sender<[[Vec<i8>; 9]; 9]>, stopChannelSender: Arc<Mutex<Sender<bool>>>, stopChannelReceiver: Arc<Mutex<Receiver<bool>>>) {
    let mut i = 0;
    let mut j = 0;
    // Find the first cell without a definitive value, if it doesn't exist then "i" will reach the value 9
    while i < 9 && sudoku[i][j].len() == 1 {
        // If there is a signal of stop, kill the thread
        let res = stopChannelReceiver.lock().unwrap().try_recv();
        // If the operation turns out not to be ready, retry
        if let Err(e) = res {
            if e.is_empty() {
                j += 1;
                if j == 9 {
                    i += 1;
                    j = 0;
                }
            }
        } else if res.unwrap() == true {
            // Send a stop signal to the running threads, because the signal received has been consumed.
            // If there isn't a running thread, an error will be returned because the channel receiver will be destroyed. Since this is not a problem, the case isn't handled.
            stopChannelSender.lock().unwrap().send(true);
            return;
        }
    }
    // If "i" has value 9 then the sudoku is already a solution, so it can be saved in the solution channel.
	// If there is already a solution in the channel, this upload will be ignored by the receiver
    if i == 9 {
        solutionChannel.send(sudoku).unwrap();
    } else {
        crossbeam::scope(|scope| {
            // For every possible value of the found cell, copy the matrix, fix that value and try to find a solution
            for z in 0..sudoku[i][j].len() {
                // If there is a signal of stop, kill the goroutine
                let res = stopChannelReceiver.lock().unwrap().try_recv();
                // If the operation turns out not to be ready, retry
                if let Err(e) = res {
                    if e.is_empty() {
                        // Copy the sudoku matrix
                        let mut sudokuCopy = copySudokuMatrix(&sudoku);
                        // Fix the value
                        sudokuCopy[i][j] = vec![sudoku[i][j][z]];
                        // Delete the contraints, in order to converge faster to a solution
                        sudokuCopy = sudokuConstraintsElimination(sudokuCopy);
                        // Before starting the new goroutine, check if the obtained matrix is correct
                        let receiverClone = stopChannelReceiver.clone();
                        let senderClone = stopChannelSender.clone();
                        if CheckSudokuMethods::checkBaseSudokuCorrectness(copySudokuMatrix(&sudokuCopy)) {
                            // Start a new thread that executes searchSolution with the new matrix
                            scope.spawn(move |_var| {searchSolution(sudokuCopy, &solutionChannel, senderClone, receiverClone)});
                        }
                    }
                } else if res.unwrap() == true {
                    // Send a stop signal to the running threads, because the signal received has been consumed.
                    // If there isn't a running thread, an error will be returned because the channel receiver will be destroyed. Since this is not a problem, the case isn't handled.
                    stopChannelSender.lock().unwrap().send(true);
                    return;
                }
            }
        });
    }
}

// Copy the sudoku matrix, copying every integer value
fn copySudokuMatrix(sudoku: &[[Vec<i8>; 9]; 9]) -> [[Vec<i8>; 9]; 9] {
    let mut sudokuCopy: [[Vec<i8>; 9]; 9] = Default::default();
    for i in 0..9 {
        for j in 0..9 {
            for z in 0..sudoku[i][j].len() {
                sudokuCopy[i][j].push(sudoku[i][j][z]);
            }
        }
    }
    return sudokuCopy;
}

fn countPossibilities(sudoku: [[Vec<i8>; 9]; 9]) -> BigInt {
    let mut total: BigInt = One::one();
    for i in 0..9 {
        for j in 0..9 {
            total *= BigInt::from(sudoku[i][j].len());
        }
    }
    return total;
}