package year2024_day5

import (
	"fmt"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day5() {
	fmt.Println("Day 5")
	pageOrderingRules, updates := parseInput("example.txt")

	for _, rule := range pageOrderingRules {
		utils.Log("Rule: ", rule)
	}

	for _, update := range updates {
		utils.Log("Update: ", update)
	}
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
