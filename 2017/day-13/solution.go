package main

import "fmt"

func read_input() map[int]int {
	var level, depth int

	var result = make(map[int]int)

	for {
		if n, err := fmt.Scanf("%d: %d", &level, &depth); n != 2 || err != nil {
			break
		}

		result[level] = depth
	}

	return result
}

func penalty(firewalls map[int]int) int {
	var score = 0

	for level, depth := range firewalls {
		if level % (2 * depth - 2) == 0 {
			score += level * depth
		}
	}

	return score
}

func caught(firewalls map[int]int, offset int) bool {
	for level, depth := range firewalls {
		if (level + offset) % (2 * depth - 2) == 0 {
			return true
		}
	}

	return false
}

func main() {
	firewalls := read_input()

	fmt.Printf("Penalty: %d\n", penalty(firewalls))

	var offset = 1

	for ; caught(firewalls, offset); offset++ {
	}

	fmt.Printf("Wait time: %d\n", offset)
}
