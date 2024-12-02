package main

import (
	"flag"

	_2024_day1 "github.com/mansdahlstrom1/advent-of-code/go/src/_2024/day1"
	_2024_day2 "github.com/mansdahlstrom1/advent-of-code/go/src/_2024/day2"
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
		_2024_day1.Day1()
	case 2:
		_2024_day2.Day2()
	default:
		println("Please provide a valid day")
	}
}
