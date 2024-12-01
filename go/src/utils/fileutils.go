package utils

import (
	"os"
	"path/filepath"
)

func ReadFile (packageName string, day string, filename string) string {
    cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	filePath := filepath.Join(cwd, "src", packageName, day, filename)
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
