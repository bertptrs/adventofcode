package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"sort"
	"strconv"
)

type SetEntry struct {
	parent, size int
}

type DisjointSet struct {
	set []SetEntry
}

func NewDisjointSet(size int) *DisjointSet {
	set := make([]SetEntry, size)

	for i := range len(set) {
		set[i].parent = i
		set[i].size = 1
	}

	instance := new(DisjointSet)
	instance.set = set

	return instance
}

func (d *DisjointSet) Find(item int) int {
	for d.set[item].parent != item {
		d.set[item].parent = d.set[d.set[item].parent].parent
		item = d.set[item].parent
	}

	return item
}

func (d *DisjointSet) Union(x, y int) bool {
	xp := d.Find(x)
	yp := d.Find(y)

	if xp == yp {
		return false
	}

	if yp < xp {
		xp, yp = yp, xp
	}

	d.set[xp].size += d.set[yp].size
	d.set[yp].parent = xp

	return true
}

func (d *DisjointSet) Size(item int) int {
	if d.set[item].parent == item {
		return d.set[item].size
	} else {
		return 0
	}
}

type DistanceHeap [][3]int

func (h DistanceHeap) Len() int           { return len(h) }
func (h DistanceHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h DistanceHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *DistanceHeap) Push(x any) {
	// Push and Pop use pointer receivers because they modify the slice's length,
	// not just its contents.
	*h = append(*h, x.([3]int))
}

func (h *DistanceHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func read_input(filename string) [][3]int {
	var points [][3]int
	var point [3]int

	file, err := os.Open(filename)

	if err != nil {
		panic(err)
	}

	defer file.Close()
	reader := bufio.NewReader(file)

	for {
		parsed, err := fmt.Fscanf(reader, "%d,%d,%d\n", &point[0], &point[1], &point[2])
		if err != nil || parsed != 3 {
			break
		}
		points = append(points, point)
	}

	return points
}

func usage() {
	fmt.Printf("Usage: %v INPUT_FILE [connections]\n", os.Args[0])
	os.Exit(1)
}

func compute_group_sizes(groups *DisjointSet, len int) []int {
	var sizes []int
	for i := range len {
		size := groups.Size(i)
		if size > 0 {
			sizes = append(sizes, size)
		}
	}

	sort.Ints(sizes)

	return sizes
}

func main() {
	if len(os.Args) < 2 {
		usage()
	}

	connections := 1000
	if len(os.Args) >= 3 {
		parsed, err := strconv.Atoi(os.Args[2])
		if err != nil {
			usage()
		}
		connections = parsed
	}

	points := read_input(os.Args[1])

	distances := make([][3]int, 0, len(points)*(len(points)-1)/2)

	for i, a := range points {
		for j := i + 1; j < len(points); j += 1 {
			b := points[j]

			square_dist := (a[0]-b[0])*(a[0]-b[0]) + (a[1]-b[1])*(a[1]-b[1]) + (a[2]-b[2])*(a[2]-b[2])

			distances = append(distances, [3]int{square_dist, i, j})
		}
	}

	size_heap := DistanceHeap(distances)
	heap.Init(&size_heap)

	groups := NewDisjointSet(len(points))

	for range connections {
		first := heap.Pop(&size_heap)
		d := first.([3]int)
		groups.Union(d[1], d[2])
	}

	sizes := compute_group_sizes(groups, len(points))

	product := 1
	for i := len(sizes) - 3; i < len(sizes); i += 1 {
		product *= sizes[i]
	}

	fmt.Printf("Part 1: %v\n", product)

	to_merge := len(sizes) - 1
	for {
		first := heap.Pop(&size_heap)
		d := first.([3]int)
		if groups.Union(d[1], d[2]) {
			to_merge -= 1
			if to_merge == 0 {
				fmt.Printf("Part 2: %v\n", points[d[1]][0]*points[d[2]][0])
				return
			}
		}
	}
}
