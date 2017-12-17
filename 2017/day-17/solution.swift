let input = Int(readLine()!)!

var index = 0

var buffer = [0];

for i in 1...2017 {
	index = ((index + input) % buffer.count) + 1
	buffer.insert(i, at: index)
}

print(buffer[buffer.index(of: 2017)! + 1])

var last = 0

index = 0

for i in 1...50000000 {
	index = ((index + input) % i) + 1
	if index == 1 {
		last = i
	}
}

print(last)
