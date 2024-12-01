package _2024

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

func Day1() {
	fmt.Println("Day 1 Solution")

	var files = []string{"example", "input_pt1"}

	for _, file := range files {
		left, right := parseInput(file)
		sort.Slice(left, func(i, j int) bool {
			return left[i] < left[j]
		})
		sort.Slice(right, func(i, j int) bool {
			return right[i] < right[j]
		})

		fmt.Printf("Total distance for file %s: %d \n", file, calculateDistance(left, right))
	}
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

func calculateDistance(left []int, right []int) int {
	var totalDistance = 0
	for i, val := range left {
		var distance = right[i] - val
		totalDistance += utils.Abs(distance)
	}

	return totalDistance
}
