package year2024_day5

import (
	"fmt"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day5() {
	fmt.Println("Day 5")
	pageOrderingRules, updates := parseInput("input.txt")

	pt1 := part1(pageOrderingRules, updates)
	fmt.Println("Part 1 example.txt: ", pt1)

	pt2 := part2(pageOrderingRules, updates)
	fmt.Println("Part 2 example.txt: ", pt2)
}

func part1(pageOrderingRules [][2]int, updates [][]int) int {
	var sum = 0
	for _, pagesToUpdate := range updates {
		utils.Log("pagesToUpdate: ", pagesToUpdate)

		var validUpdate, _ = checkPagesToUpdate(pageOrderingRules, pagesToUpdate, false)
		if validUpdate {
			middleIndex := len(pagesToUpdate) / 2
			sum += pagesToUpdate[middleIndex]
		}
	}

	return sum
}

func part2(pageOrderingRules [][2]int, updates [][]int) int {
	var sum = 0
	for _, pagesToUpdate := range updates {
		utils.Log("pagesToUpdate: ", pagesToUpdate)

		var validUpdate, _ = checkPagesToUpdate(pageOrderingRules, pagesToUpdate, false)
		if !validUpdate {
			// do stuff
			afterCheckingAgain, orderedPages := checkPagesToUpdate(pageOrderingRules, pagesToUpdate, true)
			if afterCheckingAgain {
				utils.Log("After checking again, the pages are in order", orderedPages)
				middleIndex := len(orderedPages) / 2
				sum += orderedPages[middleIndex]
			}
		}
	}

	return sum
}

func lookBehind(pagesToUpdate []int, pageIndex int, pageThatCannotBeBefore int) int {
	for i := pageIndex; i >= 0; i-- {
		if pagesToUpdate[i] == pageThatCannotBeBefore {
			utils.Log("Found page that cannot be before...", pageThatCannotBeBefore, "Exiting")
			return i
		}
	}
	return -1
}

func checkPagesToUpdate(pageOrderingRules [][2]int, pagesToUpdate []int, shouldTryToFix bool) (bool, []int) {
	for pageIndex, pageNumber := range pagesToUpdate {
		utils.Log("Page number: ", pageNumber)
		for _, rule := range pageOrderingRules {
			if rule[0] == pageNumber {
				pageThatCannotBeBefore := rule[1]
				utils.Log("Found page number in rule: ", rule, "as index: ", 0, "checking that", pageThatCannotBeBefore, "is not before")
				offendingIndex := lookBehind(pagesToUpdate, pageIndex, pageThatCannotBeBefore)
				if offendingIndex > -1 {

					if shouldTryToFix {
						utils.Log("Trying to fix the page ordering of ...", pagesToUpdate, "pageIndex is", pageIndex, "amd", pageThatCannotBeBefore, "at index", offendingIndex)
						fixedPageUpdates := make([]int, len(pagesToUpdate))
						var realIndex = 0
						for i, page := range pagesToUpdate {
							if i == offendingIndex {
								utils.Log("Skipping", page)
								continue
							}

							utils.Log("Adding", page, "to fixedPageUpdates", realIndex)
							fixedPageUpdates[realIndex] = page

							if realIndex+1 == pageIndex {
								realIndex++
								utils.Log("Adding", pageThatCannotBeBefore, "to fixedPageUpdates", realIndex)
								fixedPageUpdates[realIndex] = pageThatCannotBeBefore
							}
							realIndex++

						}
						utils.Log("Fixed page ordering to ...", fixedPageUpdates)
						return checkPagesToUpdate(pageOrderingRules, fixedPageUpdates, true)
					}

					return false, pagesToUpdate
				}
			}
		}
	}

	fmt.Println("All pages are in order are now in order... returning", pagesToUpdate)
	return true, pagesToUpdate
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
