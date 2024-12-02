package _2024_day2

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day2() {
	fmt.Println("Day 2 Solution")

	var files = []string{"example", "input"}
	for _, filename := range files {
		reports := parseInput(filename)

		numberOfSafeReports := part1(reports)
		fmt.Printf("Number of safe reports for file %s: %d\n", filename, numberOfSafeReports)
	}
}

func part1(reports [][]int) int {
	var numberOfSafeReports int = 0

	for _, report := range reports {
		fmt.Println(report)

		var prevLevel int = report[0]
		var isIncreasing bool = report[1] > report[0]
		var isSafe bool = true
		for i, level := range report {
			if i == 0 {
				continue
			}

			if utils.Abs(prevLevel-level) > 3 {
				fmt.Printf("Not safe because difference between %d and %d is greater the 3\n", prevLevel, level)
				isSafe = false
				break
			}

			if isIncreasing {
				if level <= prevLevel {
					fmt.Printf("Not safe because not increasing anymore, %d => %d\n", prevLevel, level)
					isSafe = false
					break
				}
			} else {
				if level >= prevLevel {
					fmt.Printf("Not safe because not decreasing anymore, %d => %d\n", prevLevel, level)
					isSafe = false
					break
				}
			}

			prevLevel = level
		}

		if isSafe {
			numberOfSafeReports++
		}
	}
	return numberOfSafeReports
}

func parseInput(filename string) [][]int {
	var text = utils.ReadFile("_2024", "day2", filename)
	rows := strings.Split(text, "\n")

	var reports [][]int = make([][]int, len(rows)-1) // Remove last empty row

	for rowIndex, row := range rows {
		if row == "" {
			continue
		}
		levels := strings.Split(row, " ")
		integers := make([]int, len(levels))
		for levelIndex, level := range levels {
			n, err := strconv.Atoi(level)
			if err != nil {
				panic(err)
			}
			integers[levelIndex] = n
		}
		reports[rowIndex] = integers
	}

	return reports
}
