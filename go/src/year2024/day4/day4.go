package year2024_day4

import (
	"fmt"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day4() {
	fmt.Println("Day 4")

	input := utils.ReadFile("year2024", "day4", "input.txt")
	rows := strings.Split(input, "\n")

	// Remove the last empty row due to line break at end of file
	rows = rows[:len(rows)-1]
	part1(rows)
}

func part1(rows []string) {
	var rowsWithXMAS = 0
	for _, row := range rows {
		rowsWithXMAS += strings.Count(row, "XMAS")
		rowsWithXMAS += strings.Count(row, "SAMX")
	}

	// Assume all rows have the same length
	var columns []string = make([]string, len(rows[0]))
	for _, row := range rows {
		for i, char := range row {
			columns[i] += string(char)
		}
	}

	var columnsWithXMAS = 0
	for _, column := range columns {
		columnsWithXMAS += strings.Count(column, "XMAS")
		columnsWithXMAS += strings.Count(column, "SAMX")
	}

	var maxColumnIndex = len(rows) - 1
	utils.Log("Max column Index: ", maxColumnIndex)
	var maxRowIndex = len(rows[0]) - 1
	utils.Log("Max row Index: ", maxRowIndex)

	var diagonalsWithXMAS = 0
	for y, row := range rows {
		for x, char := range row {
			if char != 'X' {
				continue
			}

			var topIsSafe = y > 2                   // 0, 1, 2 are ok
			var leftIsSafe = x > 2                  // 0, 1, 2 are ok
			var bottomIsSafe = y < maxColumnIndex-2 // 137, 138, 139 are ok (given maxColumnIndex is 139)
			var rightIsSafe = x < maxRowIndex-2     // 137, 138, 139 are ok (given maxRowIndex is 139)

			// 1. top right
			if topIsSafe && rightIsSafe && checkTopRight(rows, x, y, "MAS") {
				utils.Log("Found top right going XMAS at: ", x, y)
				diagonalsWithXMAS++
			}
			// 2. bottom right
			if bottomIsSafe && rightIsSafe && checkBottomRight(rows, x, y, "MAS") {
				utils.Log("Found bottom right going XMAS at: ", x, y)
				diagonalsWithXMAS++
			}

			// 3. bottom left
			if bottomIsSafe && leftIsSafe && checkBottomLeft(rows, x, y, "MAS") {
				utils.Log("Found bottom left going XMAS at: ", x, y)
				diagonalsWithXMAS++
			}

			// 4. top left
			if topIsSafe && leftIsSafe && checkTopLeft(rows, x, y, "MAS") {
				utils.Log("Found top left going XMAS at: ", x, y)
				diagonalsWithXMAS++
			}
		}
	}
	utils.Log("Max column Index: ", maxColumnIndex)
	utils.Log("Max row Index: ", maxRowIndex)

	utils.Log("Rows with XMAS: ", rowsWithXMAS)
	utils.Log("Columns rows with XMAS: ", columnsWithXMAS)
	utils.Log("diagonals with XMAS: ", diagonalsWithXMAS)

	fmt.Println("Total XMAS: ", rowsWithXMAS+columnsWithXMAS+diagonalsWithXMAS)
}

func checkTopRight(rows []string, x int, y int, search string) bool {
	return rows[y-1][x+1] == search[0] && rows[y-2][x+2] == search[1] && rows[y-3][x+3] == search[2]
}

func checkBottomRight(rows []string, x int, y int, search string) bool {
	return rows[y+1][x+1] == search[0] && rows[y+2][x+2] == search[1] && rows[y+3][x+3] == search[2]
}

func checkBottomLeft(rows []string, x int, y int, search string) bool {
	return rows[y+1][x-1] == search[0] && rows[y+2][x-2] == search[1] && rows[y+3][x-3] == search[2]
}

func checkTopLeft(rows []string, x int, y int, search string) bool {

	return rows[y-1][x-1] == search[0] && rows[y-2][x-2] == search[1] && rows[y-3][x-3] == search[2]
}
