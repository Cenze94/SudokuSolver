from PyQt5.QtWidgets import QHBoxLayout, QPushButton, QFileDialog
from PyQt5.QtCore import Qt
from PyQt5.QtGui import QFont
import Main
import SudokuManager
import AppTerminal
import shutil
import os


class ButtonsLayout:
    loadButtonText = 'Load sudoku'
    startButtonText = 'Start solving'
    continueButtonText = 'Continue'
    resetSudokuButtonText = 'Reset sudoku'
    buttonHeight = 20
    buttonWidth = 200
    loadButton = None
    startButton = None

    @staticmethod
    def getButtonsLayout():
        layout = QHBoxLayout()

        font = QFont("Arial", 11, QFont.DemiBold)

        ButtonsLayout.loadButton = QPushButton(ButtonsLayout.loadButtonText)
        loadButton = ButtonsLayout.loadButton
        loadButton.setFixedWidth(200)
        loadButton.setFixedHeight(50)
        loadButton.setFont(font)
        loadButton.clicked.connect(ButtonsLayout.loadSudoku)
        layout.addWidget(loadButton, 0, Qt.AlignLeft)

        ButtonsLayout.startButton = QPushButton(ButtonsLayout.startButtonText)
        startButton = ButtonsLayout.startButton
        startButton.setFixedWidth(200)
        startButton.setFixedHeight(50)
        startButton.setFont(font)
        startButton.clicked.connect(ButtonsLayout.startSolving)
        layout.addWidget(startButton, 0, Qt.AlignRight)

        return layout

    # Method invoked when pressing "Load sudoku" button
    @staticmethod
    def loadSudoku():
        options = QFileDialog.Options()
        options |= QFileDialog.DontUseNativeDialog
        filename, _ = QFileDialog.getOpenFileName(Main.appReference, "Select Sudoku File", "",
                                                  "Text Files (*.txt);;All files (*)", options=options)
        if filename:
            if SudokuManager.loadSudokuFromFile(filename):
                AppTerminal.AppTerminal.addText("Sudoku successfully loaded.")
                if os.path.abspath("LastSudoku.txt") != os.path.normpath(filename):
                    shutil.copyfile(filename, "LastSudoku.txt")
            else:
                AppTerminal.AppTerminal.addText("Invalid file.")

    # Method invoked when pressing "Start solving" button
    @staticmethod
    def startSolving():
        if SudokuManager.sudoku:
            ButtonsLayout.loadButton.setEnabled(False)
            startButton = ButtonsLayout.startButton
            startButton.setText(ButtonsLayout.continueButtonText)
            startButton.clicked.connect(ButtonsLayout.continueSolving)
            startButton.setEnabled(False)

            if Main.go:
                print("Go!")
            else:
                print("Rust!")
        else:
            AppTerminal.AppTerminal.addText("Load a valid sudoku.")

    # Method invoked when a step has been computed and the "Continue" button must be enabled
    @staticmethod
    def endedStep():
        ButtonsLayout.startButton.setEnabled(True)

    # Method invoked when pressing "Continue" button
    @staticmethod
    def continueSolving():
        ButtonsLayout.startButton.setEnabled(False)

    # Method invoked when the sudoku has been solved and the old sudoku has to be reloaded
    @staticmethod
    def endedSolving():
        startButton = ButtonsLayout.startButton
        startButton.setText(ButtonsLayout.resetSudokuButtonText)
        startButton.clicked.connect(ButtonsLayout.resetSudoku)
        ButtonsLayout.startButton.setEnabled(True)

    # Method invoked to reload the old sudoku
    @staticmethod
    def resetSudoku():
        ButtonsLayout.loadButton.setEnabled(True)
        startButton = ButtonsLayout.startButton
        startButton.setText(ButtonsLayout.startButtonText)
        startButton.clicked.connect(ButtonsLayout.startSolving)
        SudokuManager.loadSudokuFromFile("LastSudoku.txt")
