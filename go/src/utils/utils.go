package utils

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func ReadFile(packageName string, day string, filename string) string {
	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	// UGly hack, fix in upcoming day
	section := strings.Split(cwd, "advent-of-code/go")
	filePath := filepath.Join(section[0], "advent-of-code/go", "src", packageName, day, filename)
	text, err := os.ReadFile(filePath)
	if err != nil {
		panic(err)
	}

	return string(text)
}

// Abs returns the absolute value of x.
func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func DeleteElementAtIndex(slice []int, index int) []int {
	newSlice := make([]int, len(slice))
	copy(newSlice, slice)
	return append(newSlice[:index], newSlice[index+1:]...)
}

var Debug = true // Set to false to suppress logs

func Log(args ...interface{}) {
	if Debug {
		fmt.Println(args...)
	}
}

func ParseInt(numberAsString string) int {
	n, err := strconv.Atoi(numberAsString)
	if err != nil {
		panic(err)
	}
	return n
}
