package year2024_day3

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day3() {
	utils.Log("Day 3 Solution")

	var files = []string{"example.txt", "example2.txt", "input.txt"}
	for _, filename := range files {
		var corruptMemory = utils.ReadFile("year2024", "day3", filename)

		sum1 := part1(corruptMemory)
		fmt.Println("Part 1 sum: ", sum1)

		sum2 := part2(corruptMemory)
		fmt.Println("Part 2 sum: ", sum2)
	}
}

func part1(corruptMemory string) int {
	r, _ := regexp.Compile(`mul\(\d{1,3},\d{1,3}\)`)
	allMul := r.FindAllString(corruptMemory, -1)

	var sum int = 0
	for _, expression := range allMul {
		utils.Log(expression)
		var substring string = expression

		substring = strings.TrimPrefix(substring, "mul(")
		substring = strings.TrimSuffix(substring, ")")
		values := strings.Split(substring, ",")

		n1 := utils.ParseInt(values[0])
		n2 := utils.ParseInt(values[1])

		utils.Log(n1 * n2)
		sum += n1 * n2
	}

	return sum
}

func part2(corruptMemory string) int {
	r, _ := regexp.Compile(`don't\(\).*do\(\)`)
	withoutDisabledOperations := r.ReplaceAllString(corruptMemory, "")

	return part1(withoutDisabledOperations)
}
