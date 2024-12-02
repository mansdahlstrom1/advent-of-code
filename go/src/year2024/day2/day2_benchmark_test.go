package year2024_day2

import (
	"testing"

	"github.com/mansdahlstrom1/advent-of-code/go/src/utils"
)

// BenchmarkPart1 benchmarks the part1 function
func BenchmarkPart1(b *testing.B) {
	reports := parseInput("input.txt") // Ensure valid input

	// Suppress output
	utils.Debug = false // Disable logs
	defer func() { utils.Debug = true }()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		part1(reports)
	}
}

// BenchmarkPart2 benchmarks the part2 function
func BenchmarkPart2(b *testing.B) {
	reports := parseInput("input.txt") // Ensure valid input

	// Suppress output
	utils.Debug = false // Disable logs
	defer func() { utils.Debug = true }()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		part2(reports)
	}
}
