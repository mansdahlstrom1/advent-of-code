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
	numberOfXmas := part1(rows)
	fmt.Println("Part 1 - Total XMAS: ", numberOfXmas)

	numberOfMasAsX := part2(rows)
	fmt.Println("Part 2 - Total XMAS: ", numberOfMasAsX)
}

func part1(rows []string) int {
	var rowsWithXMAS = 0
	for _, row := range rows {
		rowsWithXMAS += strings.Count(row, "XMAS")
		rowsWithXMAS += strings.Count(row, "SAMX")
	}

	// Assume all rows have the same length
	var columns []string = getColumns(rows)
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
			if topIsSafe && rightIsSafe && checkDirection(rows, x, y, TopRight, "MAS") {
				utils.Log("Found top right going XMAS at: ", x, y)
				diagonalsWithXMAS++
			}
			// 2. bottom right
			if bottomIsSafe && rightIsSafe && checkDirection(rows, x, y, BottomRight, "MAS") {
				utils.Log("Found bottom right going XMAS at: ", x, y)
				diagonalsWithXMAS++
			}

			// 3. bottom left
			if bottomIsSafe && leftIsSafe && checkDirection(rows, x, y, BottomLeft, "MAS") {
				utils.Log("Found bottom left going XMAS at: ", x, y)
				diagonalsWithXMAS++
			}

			// 4. top left
			if topIsSafe && leftIsSafe && checkDirection(rows, x, y, TopLeft, "MAS") {
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

	// 2454
	return rowsWithXMAS + columnsWithXMAS + diagonalsWithXMAS
}

func part2(rows []string) int {
	var maxColumnIndex = len(rows) - 1
	var maxRowIndex = len(rows[0]) - 1
	var numberOfMasAsX = 0
	for y, row := range rows {
		for x, char := range row {
			if char != 'A' {
				continue
			}

			var topIsSafe = y > 0
			var leftIsSafe = x > 0
			var bottomIsSafe = y < maxColumnIndex
			var rightIsSafe = x < maxRowIndex

			if !topIsSafe || !rightIsSafe || !leftIsSafe || !bottomIsSafe {
				continue
			}

			utils.Log("Checking for X of MAS at: ", x, y)

			// 1. top right / bottom left
			bottomLeftChar := rune(rows[y+1][x-1])
			matchTopRight := bottomLeftChar == 'M' && checkDirection(rows, x-1, y+1, TopRight, "AS")
			topRightChar := rune(rows[y-1][x+1])
			matchBottomLeft := topRightChar == 'M' && checkDirection(rows, x+1, y-1, BottomLeft, "AS")

			if !matchTopRight && !matchBottomLeft {
				continue
			}

			// 2. top left / bottom right
			topLeftChar := rune(rows[y-1][x-1])
			matchBottomRight := topLeftChar == 'M' && checkDirection(rows, x-1, y-1, BottomRight, "AS")
			bottomRightChar := rune(rows[y+1][x+1])
			matchTopLeft := bottomRightChar == 'M' && checkDirection(rows, x+1, y+1, TopLeft, "AS")

			if !matchBottomRight && !matchTopLeft {
				continue
			}

			utils.Log("Found X of \"MAS\" going MAS at: ", x, y)
			numberOfMasAsX++

		}
	}

	// 1858
	return numberOfMasAsX
}

func getColumns(rows []string) []string {
	// Assume all rows have the same length
	var columns []string = make([]string, len(rows[0]))
	for _, row := range rows {
		for i, char := range row {
			columns[i] += string(char)
		}
	}

	return columns
}

type Direction int

const (
	TopRight Direction = iota
	BottomRight
	BottomLeft
	TopLeft
)

func checkDirection(rows []string, x int, y int, direction Direction, search string) bool {
	var containsSearch = true

	nextY := y
	nextX := x
	for i, char := range search {
		jump := i + 1
		switch direction {
		case TopRight:
			nextY = y - jump
			nextX = x + jump
		case BottomRight:
			nextY = y + jump
			nextX = x + jump
		case BottomLeft:
			nextY = y + jump
			nextX = x - jump
		case TopLeft:
			nextY = y - jump
			nextX = x - jump
		}
		if rune(rows[nextY][nextX]) != char {
			containsSearch = false
			break
		}
	}
	return containsSearch
}
