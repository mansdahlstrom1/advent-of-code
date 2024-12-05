package year2024_day5

import (
	"fmt"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day5() {
	fmt.Println("Day 5")
	pageOrderingRules, updates := parseInput("input.txt")

	sum := part1(pageOrderingRules, updates)
	fmt.Println("Part 1 example.txt: ", sum)
}

func part1(pageOrderingRules [][2]int, updates [][]int) int {
	var sum = 0
	for _, pagesToUpdate := range updates {
		utils.Log("pagesToUpdate: ", pagesToUpdate)

		var validUpdate = checkPagesToUpdate(pageOrderingRules, pagesToUpdate)
		if validUpdate {
			middleIndex := len(pagesToUpdate) / 2
			sum += pagesToUpdate[middleIndex]
		}
	}

	return sum
}

func isPageBehind(pagesToUpdate []int, pageIndex int, pageThatCannotBeBefore int) bool {
	for i := pageIndex; i >= 0; i-- {
		if pagesToUpdate[i] == pageThatCannotBeBefore {
			utils.Log("Found page that cannot be before...", pageThatCannotBeBefore, "Exiting")
			return true
		}
	}
	return false
}

func checkPagesToUpdate(pageOrderingRules [][2]int, pagesToUpdate []int) bool {
	for pageIndex, pageNumber := range pagesToUpdate {
		utils.Log("Page number: ", pageNumber)
		for _, rule := range pageOrderingRules {
			if rule[0] == pageNumber {
				pageThatCannotBeBefore := rule[1]
				utils.Log("Found page number in rule: ", rule, "as index: ", 0, "checking that", pageThatCannotBeBefore, "is not before")
				if isPageBehind(pagesToUpdate, pageIndex, pageThatCannotBeBefore) {
					return false
				}
			}
		}
	}
	return true
}

func parseInput(filename string) ([][2]int, [][]int) {
	text := utils.ReadFile("year2024", "day5", filename)
	parts := strings.Split(text, "\n\n")
	part1 := strings.Split(parts[0], "\n")
	part2 := strings.Split(parts[1], "\n")

	var pageOrderingRules = make([][2]int, len(part1))
	for i, rule := range part1 {
		ruleDetails := strings.Split(rule, "|")
		pageOrderingRules[i] = [2]int{utils.ParseInt(ruleDetails[0]), utils.ParseInt(ruleDetails[1])}
	}
	// -1 for the \n at the end of the file...
	var updates = make([][]int, len(part2)-1)
	for i, update := range part2 {
		if update == "" {
			continue
		}
		numbersAsString := strings.Split(update, ",")
		updates[i] = make([]int, len(numbersAsString))
		for j, nas := range numbersAsString {
			updates[i][j] = utils.ParseInt(nas)
		}
	}

	return pageOrderingRules, updates
}
