package main

import (
	"flag"

	"github.com/mansdahlstrom1/advent-of-code/go/src/_2024"
)

var day int
func init() {
	flag.IntVar(&day, "day", 0, "Please provide a valid day")
}

func main() {
    flag.Parse();
    println("Advent of Code 2024", day)

	switch day {
	case 1:
		_2024.Day1()
    default:
        println("Please provide a valid day")
	}
}
