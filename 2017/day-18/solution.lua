#!/usr/bin/env lua

function split(input, match)
	-- Stolen from stack overflow.
	-- Why doesn't lua have string split?
	local i = 1; t = {}

	for str in string.gmatch(input, match) do
		t[i] = str
		i = i + 1
	end

	return t
end

instructions = {}
icount = 0

while true do
	line = io.read()

	if line == nil then break end

	instructions[icount] = split(line, "%S+")
	icount = icount + 1
end

registers = {}

function argval(input)
	if tonumber(input) ~= nil then
		return tonumber(input)
	elseif registers[input] ~= nil then
		return registers[input]
	else
		return 0
	end
end

iptr = 0

while true do
	instr = instructions[iptr]

	if instr[1] == "set" then
		registers[instr[2]] = argval(instr[3])
	elseif instr[1] == "snd" then
		last_played = argval(instr[2])
	elseif instr[1] == "add" then
		registers[instr[2]] = argval(instr[2]) + argval(instr[3])
	elseif instr[1] == "mul" then
		registers[instr[2]] = argval(instr[2]) * argval(instr[3])
	elseif instr[1] == "mod" then
		registers[instr[2]] = argval(instr[2]) % argval(instr[3])
	elseif instr[1] == "rcv" then
		print("Last played", last_played)
		break
	elseif instr[1] == "jgz" then
		if argval(instr[2]) > 0 then
			iptr = iptr + argval(instr[3]) - 1 -- correct for the subsequent add
		end
	else
		print("Illegal instruction", instr[1])
		break
	end

	iptr = iptr + 1
end

registers = {}
registers[0] = {}
registers[1] = {}

function argval(input, pid)
	if tonumber(input) ~= nil then
		return tonumber(input)
	elseif registers[pid][input] ~= nil then
		return registers[pid][input]
	else
		return pid
	end
end

-- Double instruction pointer
iptr = {}
iptr[0] = 0
iptr[1] = 0
-- Double receive buffer
rbuf = {}
rbuf[0] = {}
rbuf[1] = {}
-- Pointer in the receive buffer
rptr = {}
rptr[0] = 0
rptr[1] = 0
-- Number of entries in each buffer
rctr = {}
rctr[0] = 0
rctr[1] = 0
-- Whether one thread is waiting
waiting = {}
waiting[0] = false
waiting[1] = false

sends = 0

pid = 0

function nextpid(pid)
	if pid == 0 then
		return 1
	else
		return 0
	end
end

while true do
	instr = instructions[iptr[pid]]

	if waiting[0] and waiting[1] then
		print("Deadlocked!")
		break
	end

	if instr[1] == "set" then
		registers[pid][instr[2]] = argval(instr[3], pid)
	elseif instr[1] == "snd" then
		rbuf[nextpid(pid)][rctr[nextpid(pid)]] = argval(instr[2], pid)
		rctr[nextpid(pid)] = rctr[nextpid(pid)] + 1
	elseif instr[1] == "add" then
		registers[pid][instr[2]] = argval(instr[2], pid) + argval(instr[3], pid)
	elseif instr[1] == "mul" then
		registers[pid][instr[2]] = argval(instr[2], pid) * argval(instr[3], pid)
	elseif instr[1] == "mod" then
		registers[pid][instr[2]] = argval(instr[2], pid) % argval(instr[3], pid)
	elseif instr[1] == "rcv" then
		if rbuf[pid][rptr[pid]] ~= nil then
			-- Have value, read and continue
			registers[pid][instr[2]] = rbuf[pid][rptr[pid]]
			rptr[pid] = rptr[pid] + 1
			waiting[nextpid(pid)] = false
		else
			-- Need to wait for value, continue with execution of the other
			waiting[pid] = true
			pid = nextpid(pid)
			iptr[pid] = iptr[pid] - 1 -- Correct for subsequent increase
		end
	elseif instr[1] == "jgz" then
		if argval(instr[2], pid) > 0 then
			iptr[pid] = iptr[pid] + argval(instr[3], pid) - 1 -- correct for the subsequent add
		end
	else
		print("Illegal instruction", instr[1])
		break
	end

	iptr[pid] = iptr[pid] + 1
end

print("Final sends from 1:", rctr[0])
