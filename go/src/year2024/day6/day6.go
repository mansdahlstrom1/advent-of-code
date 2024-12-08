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

	pt2 := part2(grid)
	fmt.Println("Part 2: ", pt2)
}

func part1(grid [][]rune) (sum int) {
	printMap(grid)
	startX, startY := findStartPoint(grid)
	gridCopy, _, _ := traverseMap(startX, startY, Up, grid, false)

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

func part2(grid [][]rune) int {
	printMap(grid)
	startX, startY := findStartPoint(grid)
	_, _, numberOfInfinites := traverseMap(startX, startY, Up, grid, true)

	return numberOfInfinites
}

func traverseMap(startX int, startY int, initialDirection Direction, grid [][]rune, shouldLookForInfinites bool) (updatedMap [][]rune, state string, number int) {
	gridCopy := deepCopy(grid)
	utils.Log("Traversing map", startX, startY, initialDirection)
	printMap(gridCopy)

	var x = startX
	var y = startY
	var direction = initialDirection
	var numberOfInfinites = 0
	var pointsMap = make(map[string]Point)

	for isSafe(grid, x, y) {
		nextPoint := getNextCoordinates(x, y, direction)
		if !isSafe(grid, nextPoint.x, nextPoint.y) {
			gridCopy[y][x] = 'X'
			utils.Log("Walking out of bounds, breaking")
			break
		}

		inFrontOfUs := grid[nextPoint.y][nextPoint.x]
		if inFrontOfUs == '#' || inFrontOfUs == 'O' {
			// turn right if we find an obstacle
			direction = turnRight(direction)

			// Take note of point
			currentPoint := Point{x, y, direction}
			key := pointKey(currentPoint)
			if _, exists := pointsMap[key]; exists {
				// we have turned right at this point before....
				utils.Log("Found infinite loop at", x, y, direction)
				printMap(gridCopy)
				return gridCopy, "infinite-loop", numberOfInfinites
			}

			pointsMap[key] = currentPoint
		} else {
			// "commit" the move
			x = nextPoint.x
			y = nextPoint.y
			// don't overwrite the start point
			if gridCopy[y][x] != '^' {
				gridCopy[y][x] = 'X'
			}

			if shouldLookForInfinites {
				// Make a new copy of the map
				newMap := deepCopy(gridCopy)
				nextPoint = getNextCoordinates(x, y, direction)
				if isSafe(newMap, nextPoint.x, nextPoint.y) {

					// Edge case cannot place a obstacle in front of us
					inFrontOfUs = newMap[nextPoint.y][nextPoint.x]
					if inFrontOfUs == '^' {
						utils.Log("Found start point, skipping, cannot place obstacle here... the guard would notice that ")
						continue
					}

					// If next point is a wall there is no point in checking this
					if inFrontOfUs == '#' {
						utils.Log("Found wall, skipping")
						continue
					}

					if inFrontOfUs == 'X' {
						utils.Log("We've been here already, cannot place obstacle here... ")
						continue
					}

					// add a new obstacle in front of us and turn right
					newMap[nextPoint.y][nextPoint.x] = 'O'
					newDirection := turnRight(direction)

					_, state, _ = traverseMap(x, y, newDirection, newMap, false)
					if state == "infinite-loop" {
						numberOfInfinites++
					}
				}
			}
		}
	}

	return gridCopy, "out-of-bounds", numberOfInfinites
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

func getNextCoordinates(x, y int, direction Direction) Point {
	switch direction {
	case Up:
		return Point{x, y - 1, Up}
	case Right:
		return Point{x + 1, y, Right}
	case Down:
		return Point{x, y + 1, Down}
	case Left:
		return Point{x - 1, y, Left}
	}
	panic("Invalid direction")
}

func turnRight(direction Direction) Direction {
	switch direction {
	case Up:
		return Right
	case Right:
		return Down
	case Down:
		return Left
	case Left:
		return Up
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

func deepCopy(grid [][]rune) [][]rune {
	gridCopy := make([][]rune, len(grid))
	for i := range grid {
		gridCopy[i] = make([]rune, len(grid[i]))
		copy(gridCopy[i], grid[i]) // Copy the inner slice
	}

	return gridCopy
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
