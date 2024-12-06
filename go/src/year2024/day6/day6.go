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

	pt2 := part2()
	fmt.Println("Part 2 example.txt: ", pt2)
}

func part1(grid []string) int {
	gridCopy := make([][]rune, len(grid))
	for i, row := range grid {
		gridCopy[i] = []rune(row)
	}

	if utils.Debug {
		// Render the map
		for _, row := range grid {
			utils.Log(row)
		}
	}

	startX, startY := findStartPoint(grid)
	utils.Log("Start point: ", startX, startY)

	var x = startX
	var y = startY
	var direction = "up"
	var maxY = len(grid) - 1
	var maxX = len(grid[0]) - 1

	for x >= 0 && x <= maxX && y >= 0 && y <= maxY {
		// Mark where we have been with X
		gridCopy[y][x] = 'X'
		var tempX = x
		var tempY = y

		switch direction {
		case "up":
			tempY--
		case "right":
			tempX++
		case "down":
			tempY++
		case "left":
			tempX--
		}

		if tempX >= 0 && tempX <= maxX && tempY >= 0 && tempY <= maxY {
			if grid[tempY][tempX] == '#' {
				// turn if we find an obstacle
				direction = getNextDirection(direction)
			} else {
				// "commit" the move
				x = tempX
				y = tempY
			}
		} else {
			utils.Log("Walking out of bounds, breaking")
			break
		}
	}

	var numSteps = 0
	for _, row := range gridCopy {
		utils.Log(string(row))
		for _, char := range row {
			if char == 'X' {
				numSteps++
			}
		}
	}

	return numSteps
}

func part2() int {
	utils.Log("Part 2")
	return 0
}

func getNextDirection(direction string) string {
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

func findStartPoint(grid []string) (int, int) {
	for y, row := range grid {
		for x, char := range row {
			if char == '^' {
				return x, y
			}
		}
	}
	panic("Could not find start point")
}

func parseInput(filename string) []string {
	utils.Log("Parsing input")
	text := utils.ReadFile("year2024", "day6", filename)
	rows := strings.Split(text, "\n")

	// remove last row if empty
	if rows[len(rows)-1] == "" {
		rows = rows[:len(rows)-1]
	}

	return rows
}
