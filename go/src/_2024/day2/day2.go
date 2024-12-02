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

			isSafe = checkSafety(prevLevel, level, isIncreasing)
			if !isSafe {
				break
			}

			prevLevel = level
		}

		if isSafe {
			numberOfSafeReports++
		}
	}
	return numberOfSafeReports
}

func checkSafety(prevLevel, level int, isIncreasing bool) bool {
	if utils.Abs(prevLevel-level) > 3 {
		return false
	}

	if isIncreasing {
		if level <= prevLevel {
			return false
		}
	} else {
		if level >= prevLevel {
			return false
		}
	}

	return true
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
