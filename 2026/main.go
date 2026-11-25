package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
	"time"

	"aoc2026/internal/dispatch"
)

func printUsage() {
	println("Usage:", os.Args[0], "[flags] DAY")
	os.Exit(1)
}

func main() {
	var timing bool
	var input string

	flag.BoolVar(&timing, "t", false, "Show timing output at the end")
	flag.StringVar(&input, "o", "", "File to use as input. Default to standard input")
	flag.Parse()

	if flag.NArg() != 1 {
		printUsage()
	}

	day_str := flag.Arg(0)
	day, err := strconv.Atoi(day_str)
	if err != nil {
		printUsage()
	}

	day_func := dispatch.GetFunction(day)

	if day_func == nil {
		println("Day should be 1-12")
		printUsage()
	}

	var input_file *os.File

	if input == "" {
		input_file = os.Stdin
	} else {
		file, err := os.Open(input)

		if err != nil {
			println("Failed to open input:", err)
			printUsage()
		}

		input_file = file
	}

	start := time.Now()
	part1, part2 := day_func(input_file)
	elapsed := time.Since(start)

	println(part1)
	println(part2)

	if timing {
		fmt.Printf("Elapsed: %s\n", elapsed)
	}
}
