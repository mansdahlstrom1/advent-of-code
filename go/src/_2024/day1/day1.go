package _2024_day1

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day1() {
	fmt.Println("Day 1 Solution")

	var files = []string{"example", "input"}
	for _, filename := range files {
		left, right := parseInput(filename)
		sort.Slice(left, func(i, j int) bool {
			return left[i] < left[j]
		})
		sort.Slice(right, func(i, j int) bool {
			return right[i] < right[j]
		})

		part1(filename, left, right)
		part2(filename, left, right)
	}
}

func part1(filename string, left []int, right []int) {
	var totalDistance = 0
	for i, val := range left {
		var distance = right[i] - val
		totalDistance += utils.Abs(distance)
	}

	fmt.Printf("[Part 1] - Total distance %s: %d \n", filename, totalDistance)
}

func part2(filename string, left []int, right []int) {
	var similarityScore = 0
	for _, leftValue := range left {
		var occurrences = 0
		for _, rightValue := range right {
			if leftValue == rightValue {
				occurrences++
			}
		}
		similarityScore += leftValue * occurrences
	}

	fmt.Printf("[Part 2] - Similarity score %s: %d \n", filename, similarityScore)
}

func parseInput(filename string) ([]int, []int) {
	var text = utils.ReadFile("_2024", "day1", filename)
	rows := strings.Split(text, "\n")

	var left []int
	var right []int
	for _, row := range rows {
		if row == "" {
			continue
		}

		locationIds := strings.Split(row, "   ")

		val, err := strconv.Atoi(locationIds[0])
		if err != nil {
			panic(err)
		}
		left = append(left, val)

		val, err = strconv.Atoi(locationIds[1])
		if err != nil {
			panic(err)
		}
		right = append(right, val)
	}

	return left, right
}
