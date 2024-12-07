package year2024_day6

import "strconv"

type Direction int

const (
	Up Direction = iota
	Right
	Down
	Left
)

type Point struct {
	x         int
	y         int
	direction Direction
}

// Helper function to generate a unique key for a Point
func pointKey(p Point) string {
	return strconv.Itoa(p.x) + "," + strconv.Itoa(p.y) + "," + strconv.Itoa(int(p.direction))
}
