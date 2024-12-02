package main

import (
	"flag"

	year2024_day1 "github.com/mansdahlstrom1/advent-of-code/go/src/year2024/day1"
	year2024_day2 "github.com/mansdahlstrom1/advent-of-code/go/src/year2024/day2"
)

var day int

func init() {
	flag.IntVar(&day, "day", 0, "Please provide a valid day")
}

func main() {
	flag.Parse()
	println("Advent of Code 2024", day)

	switch day {
	case 1:
		year2024_day1.Day1()
	case 2:
		year2024_day2.Day2()
	default:
		println("Please provide a valid day")
	}
}
