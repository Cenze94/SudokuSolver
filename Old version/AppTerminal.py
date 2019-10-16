from PyQt5.QtWidgets import QPlainTextEdit
from PyQt5.QtGui import QFont, QPalette
from PyQt5.QtCore import Qt


class AppTerminal:
    text = ""
    fontSize = 11
    terminalHeight = 246
    terminal = None

    @staticmethod
    def getAppTerminal():
        AppTerminal.terminal = QPlainTextEdit()
        font = QFont("Arial", AppTerminal.fontSize)
        AppTerminal.terminal.setFont(font)
        AppTerminal.terminal.setFixedHeight(AppTerminal.terminalHeight)
        palette = AppTerminal.terminal.palette()
        palette.setColor(QPalette.Base, Qt.black)
        palette.setColor(QPalette.Text, Qt.white)
        AppTerminal.terminal.setPalette(palette)
        AppTerminal.terminal.setReadOnly(True)

        return AppTerminal.terminal

    @staticmethod
    def addText(newText):
        if AppTerminal.text == "":
            AppTerminal.text += newText
        else:
            AppTerminal.text += "\n" + newText
        AppTerminal.terminal.setPlainText(AppTerminal.text)

