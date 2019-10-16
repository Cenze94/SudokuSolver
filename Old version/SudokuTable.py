from PyQt5.QtWidgets import QTableWidget, QLabel, QHeaderView, QItemDelegate, QSizePolicy
from PyQt5.QtCore import Qt
from PyQt5.QtGui import QPen, QFont
import SudokuManager


cellSize = 60


# Set table borders
class SudokuDelegate(QItemDelegate):
    def __init__(self, parent):
        super(SudokuDelegate, self).__init__(parent)

    def paint(self, QPainter, QStyleOptionViewItem, QModelIndex):
        super(SudokuDelegate, self).paint(QPainter, QStyleOptionViewItem, QModelIndex)

        QPainter.save()
        QPainter.setBrush(Qt.NoBrush)
        pen = QPen(Qt.black)
        pen.setStyle(Qt.SolidLine)

        pen.setWidthF(0.2)
        QPainter.setPen(pen)
        QPainter.drawRect(QStyleOptionViewItem.rect)

        pen.setWidth(2)
        QPainter.setPen(pen)

        if QModelIndex.row() == 2 or QModelIndex.row() == 5 or QModelIndex.row() == 8:
            rect = QStyleOptionViewItem.rect
            rectBottomLeft = rect.bottomLeft()
            rectBottomRight = rect.bottomRight()
            QPainter.drawLine(rectBottomLeft.x(), rectBottomLeft.y() + 1, rectBottomRight.x(), rectBottomRight.y() + 1)

        if QModelIndex.row() == 0:
            rect = QStyleOptionViewItem.rect
            rectTopLeft = rect.topLeft()
            rectTopRight = rect.topRight()
            QPainter.drawLine(rectTopLeft.x(), rectTopLeft.y() + 1, rectTopRight.x(), rectTopRight.y() + 1)

        if QModelIndex.column() == 2 or QModelIndex.column() == 5 or QModelIndex.column() == 8:
            rect = QStyleOptionViewItem.rect
            rectTopRight = rect.topRight()
            rectBottomRight = rect.bottomRight()
            QPainter.drawLine(rectTopRight.x(), rectTopRight.y() + 1, rectBottomRight.x(), rectBottomRight.y() + 1)

        if QModelIndex.column() == 0:
            rect = QStyleOptionViewItem.rect
            rectTopLeft = rect.topLeft()
            rectBottomLeft = rect.bottomLeft()
            QPainter.drawLine(rectTopLeft.x(), rectTopLeft.y() + 1, rectBottomLeft.x(), rectBottomLeft.y() + 1)

        QPainter.restore()


class SudokuTable:
    table = None

    @staticmethod
    def getSudokuTable():
        if SudokuTable.table is None:
            SudokuTable.setSudokuTable()
        return SudokuTable.table

    @staticmethod
    def setSudokuTable():
        SudokuTable.table = QTableWidget()
        table = SudokuTable.table
        table.setRowCount(9)
        table.setColumnCount(9)
        table.setShowGrid(False)

        vHeader = table.verticalHeader()
        vHeader.setVisible(False)

        hHeader = table.horizontalHeader()
        hHeader.setVisible(False)

        for i in range(0, 9):
            table.setColumnWidth(i, cellSize)
            table.setRowHeight(i, cellSize)
            vHeader.setSectionResizeMode(i, QHeaderView.Fixed)
            hHeader.setSectionResizeMode(i, QHeaderView.Fixed)

        table.setItemDelegate(SudokuDelegate(table))
        table.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Expanding)

        return table

    @staticmethod
    # Load sudoku into table
    def updateSudokuTable():
        # Check if there is a loaded sudoku
        if SudokuManager.sudoku:
            for i in range(0, 9):
                for j in range(0, 9):
                    if not isinstance(SudokuManager.sudoku[i][j], list):
                        SudokuTable.setFixedNumber(i, j)
                    else:
                        SudokuTable.setPossibleNumbers(i, j)

    @staticmethod
    # Set cell with a fixed number
    def setFixedNumber(i, j):
        label = QLabel()
        label.setAlignment(Qt.AlignCenter)
        font = QFont("Arial", 20)
        label.setFont(font)
        label.setText(str(SudokuManager.sudoku[i][j]))
        SudokuTable.getSudokuTable().setCellWidget(i, j, label)

    @staticmethod
    # Set cell with a list of possible numbers
    def setPossibleNumbers(it, jt):
        numbersLabel = QLabel()
        font = QFont("Arial", 13)
        numbersLabel.setFont(font)

        text = ""
        for i in range(0, 3):
            for j in range(0, 3):
                number = i * 3 + j + 1
                if number in SudokuManager.sudoku[it][jt]:
                    text += str(number)
                else:
                    text += " "
                if j < 2:
                    text += "  "
            if i < 2:
                text += "\n"

        numbersLabel.setText(text)
        numbersLabel.setAlignment(Qt.AlignCenter)
        SudokuTable.getSudokuTable().setCellWidget(it, jt, numbersLabel)

    # Update a single cell of sudoku table, which coordinates are given in input
    @staticmethod
    def updateSudokuCell(i, j):
        if isinstance(SudokuManager.sudoku[i][j], list):
            SudokuTable.setPossibleNumbers(i, j)
        else:
            SudokuTable.setFixedNumber(i, j)
