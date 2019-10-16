package main

func deleteCellValue(i, j, value int) {

}

func getCellValue(i, j int) {

}

type deleteType struct {
	i, j, value int
}

type SudokuOutputManager struct {
	Delete <-chan deleteType
}

//SudokuOutputManager run body
func (self *SudokuOutputManager) RunOutput() {
	for {
		//deleteCell := <-self.Delete
	}
}
