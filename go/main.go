package main

import (
	"flag"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
	year2024_day1 "github.com/mansdahlstrom1/advent-of-code/go/src/year2024/day1"
	year2024_day2 "github.com/mansdahlstrom1/advent-of-code/go/src/year2024/day2"
	year2024_day3 "github.com/mansdahlstrom1/advent-of-code/go/src/year2024/day3"
	year2024_day4 "github.com/mansdahlstrom1/advent-of-code/go/src/year2024/day4"
)

var day int
var debug bool

func init() {
	flag.IntVar(&day, "day", 0, "Please provide a valid day")
	flag.BoolVar(&debug, "debug", false, "Enable Debug logs")
}

func main() {
	flag.Parse()
	println("Advent of Code 2024", day)

	utils.Debug = debug

	switch day {
	case 1:
		year2024_day1.Day1()
	case 2:
		year2024_day2.Day2()
	case 3:
		year2024_day3.Day3()
	case 4:
		year2024_day4.Day4()
	default:
		println("Please provide a valid day")
	}
}
