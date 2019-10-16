import sys
from PyQt5.QtWidgets import QApplication, QWidget, QRadioButton, QDesktopWidget, QVBoxLayout, QHBoxLayout, QLabel
from PyQt5.QtCore import Qt
from PyQt5.QtGui import QFont
import SudokuTable
import Buttons
import AppTerminal
import SudokuManager


go = True
appReference = None


class App(QWidget):
    def __init__(self):
        global appReference
        super().__init__()
        appReference = self
        self.title = 'Sudoku solver'
        self.left = 10
        self.top = 10
        self.width = SudokuTable.cellSize * 9.4
        self.height = SudokuTable.cellSize * 15
        self.goButton = None
        self.rustButton = None

        self.initUI()

    def initUI(self):
        self.setWindowTitle(self.title)
        self.setGeometry(self.left, self.top, self.width, self.height)

        mainBox = QVBoxLayout()

        # Add sudoku table to the main application
        sudokuTable = SudokuTable.SudokuTable.getSudokuTable()
        sudokuTableLayout = QHBoxLayout()
        sudokuTableLayout.addWidget(sudokuTable, 0)
        mainBox.addLayout(sudokuTableLayout, 0)

        # Add buttons layout to the main application
        buttonsLayout = Buttons.ButtonsLayout.getButtonsLayout()
        mainBox.addLayout(buttonsLayout, 0)

        # Add programming language choice layout to the main application
        choiceLayout = self.getPLChoiceLayout()
        mainBox.addLayout(choiceLayout, 0)

        terminal = AppTerminal.AppTerminal.getAppTerminal()
        terminalLayout = QHBoxLayout()
        terminalLayout.addWidget(terminal, 0)
        mainBox.addLayout(terminalLayout, 0)

        SudokuManager.loadSudokuFromFile("LastSudoku.txt")

        self.setLayout(mainBox)
        self.show()

    def location_on_the_screen(self):
        ag = QDesktopWidget().availableGeometry()

        widget = self.geometry()
        x = ag.width() - ag.width()/2 - widget.width()/2
        y = ag.height() - ag.height()/2 - widget.height()/2
        self.move(x, y)

    # Set programming language choice radio buttons
    def getPLChoiceLayout(self):
        layout = QHBoxLayout()

        font = QFont("Arial", 11)

        self.goButton = QRadioButton("Go")
        self.goButton.setFont(font)
        self.goButton.setChecked(True)
        self.goButton.toggled.connect(self.checkedGoButton)

        self.rustButton = QRadioButton("Rust")
        self.rustButton.setFont(font)
        self.rustButton.toggled.connect(self.checkedRustButton)

        label = QLabel()
        font = QFont("Arial", 11, QFont.DemiBold)
        label.setFont(font)
        label.setText("Choose programming language to use:")

        layout.addWidget(label, 0)
        layout.addWidget(self.goButton, 0, Qt.AlignRight)
        layout.addWidget(self.rustButton, 0)

        return layout

    # Set "go" variable to true when "Go" radio button is checked
    def checkedGoButton(self):
        global go
        if self.goButton.isChecked():
            go = True

    # Set "go" variable to false when "Rust" radio button is checked
    def checkedRustButton(self):
        global go
        if self.rustButton.isChecked():
            go = False


if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = App()
    ex.location_on_the_screen()
    sys.exit(app.exec_())
