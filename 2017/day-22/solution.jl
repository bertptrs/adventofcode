#!/usr/bin/env julia
worldSize = 0

infected = Set()

for line in eachline(STDIN)
	for (index, c) in enumerate(line)
		coord = index - 1 + worldSize * 1im
		if c == '#'
			push!(infected, coord)
		end
	end

	worldSize += 1
end

function part1(initial, virusPos)
	infected = union(Set(), initial)
	virusDir = -1im
	infections = 0

	for i = 1:10000
		if in(virusPos, infected)
			virusDir *= 1im
			delete!(infected, virusPos)
		else
			push!(infected, virusPos)
			virusDir *= -1im
			infections += 1
		end

		virusPos += virusDir
	end

	println(infections)
end

function part2(initial, virusPos)
	state = Dict()
	virusDir = -1im

	for node in initial
		state[node] = 'I'
	end

	infections = 0

	for i = 1:10000000
		s = get(state, virusPos, 'C')

		if s == 'C'
			state[virusPos] = 'W'
			virusDir *= -1im
		elseif s == 'W'
			state[virusPos] = 'I'
			infections += 1
		elseif s == 'I'
			state[virusPos] = 'F'
			virusDir *= 1im
		else
			state[virusPos] = 'C'
			virusDir *= -1
		end

		virusPos += virusDir
	end

	println(infections)
end

virusPos = (1 + 1im) * trunc(Int, worldSize / 2)

part1(infected, virusPos)
part2(infected, virusPos)
