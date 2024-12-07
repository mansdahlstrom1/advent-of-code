package year2024_day6

import (
	"fmt"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day6() {
	fmt.Println("Day 6")
	grid := parseInput("input.txt")

	pt1 := part1(grid)
	fmt.Println("Part 1: ", pt1)
}

func part1(grid [][]rune) (sum int) {
	printMap(grid)
	startX, startY := findStartPoint(grid)
	gridCopy := traverseMap(startX, startY, "up", grid)

	var numSteps = 0
	for _, row := range gridCopy {
		for _, char := range row {
			if haveBeenHereBefore(char) {
				numSteps++
			}
		}
	}

	printMap(gridCopy)
	return numSteps
}

func traverseMap(startX int, startY int, initialDirection string, grid [][]rune) [][]rune {
	gridCopy := make([][]rune, len(grid))
	copy(gridCopy, grid)

	var x = startX
	var y = startY
	var direction = initialDirection

	for isSafe(grid, x, y) {
		tempX, tempY := getNextCoordinates(x, y, direction)
		if !isSafe(grid, tempX, tempY) {
			gridCopy[y][x] = 'X'
			utils.Log("Walking out of bounds, breaking")
			break
		}

		inFrontOfUs := grid[tempY][tempX]
		if inFrontOfUs == '#' {
			// turn right if we find an obstacle
			direction = turnRight(direction)
			continue
		}

		// "commit" the move
		x = tempX
		y = tempY
		gridCopy[y][x] = 'X'
		printMap(gridCopy)
	}

	return gridCopy
}

func haveBeenHereBefore(currentStep rune) bool {
	return currentStep == 'X' || currentStep == '^'
}

func printMap(grid [][]rune) {
	for _, row := range grid {
		utils.Log(string(row))
	}
}

func isSafe(grid [][]rune, x, y int) bool {
	var maxY = len(grid) - 1
	var maxX = len(grid[0]) - 1
	return x >= 0 && x <= maxX && y >= 0 && y <= maxY
}

func getNextCoordinates(x, y int, direction string) (nextX int, nextY int) {
	switch direction {
	case "up":
		return x, y - 1
	case "right":
		return x + 1, y
	case "down":
		return x, y + 1
	case "left":
		return x - 1, y
	}
	panic("Invalid direction")
}

func turnRight(direction string) string {
	switch direction {
	case "up":
		return "right"
	case "right":
		return "down"
	case "down":
		return "left"
	case "left":
		return "up"
	}
	panic("Invalid direction")
}

func findStartPoint(grid [][]rune) (int, int) {
	for y, row := range grid {
		for x, char := range row {
			if char == '^' {
				return x, y
			}
		}
	}
	panic("Could not find start point")
}

func parseInput(filename string) [][]rune {
	utils.Log("Parsing input")
	text := utils.ReadFile("year2024", "day6", filename)
	rows := strings.Split(text, "\n")

	// remove last row if empty
	if rows[len(rows)-1] == "" {
		rows = rows[:len(rows)-1]
	}

	var output = make([][]rune, len(rows))
	for i, row := range rows {
		output[i] = make([]rune, len(row))
		for j, char := range row {
			output[i][j] = char
		}
	}

	return output
}
