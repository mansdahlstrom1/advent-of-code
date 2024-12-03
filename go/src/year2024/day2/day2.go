package year2024_day2

import (
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day2() {
	utils.Log("Day 2 Solution")

	var files = []string{"example.txt", "input.txt"}
	for _, filename := range files {
		reports := parseInput(filename)

		numberOfSafeReportsPt1 := part1(reports)
		utils.Log("Part 1: Number of safe reports for file", filename, ": ", numberOfSafeReportsPt1)

		numberOfSafeReportsPt2 := part2(reports)
		utils.Log("Part 2: Number of safe reports for file", filename, ": ", numberOfSafeReportsPt2)
	}
}

func part1(reports [][]int) int {
	var numberOfSafeReports int = 0

	for _, report := range reports {
		isSafe := checkReportSafety(report, 0, 0)
		if isSafe {
			numberOfSafeReports++
		}
	}
	return numberOfSafeReports
}

func part2(reports [][]int) int {
	var numberOfSafeReports int = 0

	for _, report := range reports {
		isSafe := checkReportSafety(report, 0, 1)
		if isSafe {
			numberOfSafeReports++
		}
	}

	return numberOfSafeReports
}

func getIsIncreasing(report []int) bool {
	var increaseCount int = 0
	var decreaseCount int = 0

	for i, level := range report {
		if i+1 == len(report) {
			break
		}

		var nextLevel int = report[i+1]
		if nextLevel > level {
			increaseCount++
		} else if nextLevel < level {
			decreaseCount++
		}
	}

	return increaseCount >= decreaseCount
}

func checkReportSafety(report []int, retries int, retryLimit int) bool {
	utils.Log("Checking report safety", report)
	var isIncreasing bool = getIsIncreasing(report)
	isSafe, levelIndex := checkLevelSafety(report, isIncreasing)

	if isSafe {
		return true
	}

	// If there are no retries left, return false
	if retries+1 > retryLimit {
		utils.Log("Check failed... No retries left... returning false")
		return isSafe
	}

	// if we have reties left, try to remove the level that caused the issue
	// Check both possible version of the error
	utils.Log("Check failed... retrying,", retries, "of", retryLimit)
	updatedReport1 := utils.DeleteElementAtIndex(report, levelIndex)
	updatedReport2 := utils.DeleteElementAtIndex(report, levelIndex+1)

	array1Result := checkReportSafety(updatedReport1, retries+1, retryLimit)
	array2Result := checkReportSafety(updatedReport2, retries+1, retryLimit)

	// If any of the arrays are safe, return true
	return array1Result || array2Result
}

func checkLevelSafety(report []int, isIncreasing bool) (bool, int) {
	var isSafe bool = false
	for i, level := range report {
		if i+1 == len(report) {
			break
		}

		var nextLevel int = report[i+1]
		if isIncreasing {
			val := nextLevel - level
			isSafe = val > 0 && val <= 3
		} else {
			val := level - nextLevel
			isSafe = val > 0 && val <= 3
		}

		if !isSafe {
			return isSafe, i
		}
	}
	return isSafe, -1
}

func parseInput(filename string) [][]int {
	var text = utils.ReadFile("year2024", "day2", filename)
	rows := strings.Split(text, "\n")

	var reports [][]int = make([][]int, len(rows)-1) // Remove last empty row

	for rowIndex, row := range rows {
		if row == "" {
			continue
		}
		levels := strings.Split(row, " ")
		integers := make([]int, len(levels))
		for levelIndex, level := range levels {
			integers[levelIndex] = utils.ParseInt(level)
		}
		reports[rowIndex] = integers
	}

	return reports
}
