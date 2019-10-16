import SudokuTable


sudoku = []


# Load a sudoku form a text file, saving the content inside "sudoku" variable. Besides this function checks if the file
# contains a valid format, i. e. formed by digits and '_' (for missing values)
def loadSudokuFromFile(path):
    global sudoku

    with open(path, 'r') as file:
        text = file.read()
        text = text.replace('\n', '')
        text = text.replace(' ', '')

        # Get old sudoku
        oldSudoku = sudoku
        sudoku = []

        # Check text length, to avoid strange files with the first 81 valid characters
        if len(text) != 81:
            return False

        for i in range(0, 9):
            sudoku.append([])
            for j in range(0, 9):
                position = i*9 + j
                if text[position] == '_':
                    sudoku[i].append([1, 2, 3, 4, 5, 6, 7, 8, 9])
                elif text[position].isdigit():
                    sudoku[i].append(int(text[position]))
                # Character is not valid, so the file is not
                else:
                    # If the previous sudoku was valid restores it
                    if oldSudoku:
                        sudoku = oldSudoku
                    # If there wasn't a previous sudoku then empties "sudoku" variable
                    else:
                        sudoku = []
                    return False
        SudokuTable.SudokuTable.updateSudokuTable()
        return True


# Delete the possible number given in input, from the cell which coordinates are those in input. This function updates
# both the "sudoku" variable and the TableWidget
def deleteConstraint(i, j, value):
    element = sudoku[i][j]
    if isinstance(element, list):
        if value in element:
            element.remove(value)
            if len(element) == 1:
                sudoku[i][j] = element[0]
            SudokuTable.SudokuTable.updateSudokuCell(i, j)
