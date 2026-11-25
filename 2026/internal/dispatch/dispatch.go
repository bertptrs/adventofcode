package dispatch

import (
	"os"

	"aoc2026/internal/solutions/day01"
	"aoc2026/internal/solutions/day02"
	"aoc2026/internal/solutions/day03"
	"aoc2026/internal/solutions/day04"
	"aoc2026/internal/solutions/day05"
	"aoc2026/internal/solutions/day06"
	"aoc2026/internal/solutions/day07"
	"aoc2026/internal/solutions/day08"
	"aoc2026/internal/solutions/day09"
	"aoc2026/internal/solutions/day10"
	"aoc2026/internal/solutions/day11"
	"aoc2026/internal/solutions/day12"
)

type SolutionFunc = func(*os.File) (string, string)

func GetFunction(day int) SolutionFunc {
	switch day {
	case 1:
		return day01.Solve
	case 2:
		return day02.Solve
	case 3:
		return day03.Solve
	case 4:
		return day04.Solve
	case 5:
		return day05.Solve
	case 6:
		return day06.Solve
	case 7:
		return day07.Solve
	case 8:
		return day08.Solve
	case 9:
		return day09.Solve
	case 10:
		return day10.Solve
	case 11:
		return day11.Solve
	case 12:
		return day12.Solve
	}

	return nil
}
