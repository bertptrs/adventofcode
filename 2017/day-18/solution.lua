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

function argval(input, registers)
	if tonumber(input) ~= nil then
		return tonumber(input)
	elseif registers[input] ~= nil then
		return registers[input]
	else
		return 0
	end
end

function alu(instr, registers)
	-- Perform instruction and return offset to iptr
	if instr[1] == "set" then
		registers[instr[2]] = argval(instr[3], registers)
	elseif instr[1] == "snd" then
		last_played = argval(instr[2], registers)
	elseif instr[1] == "add" then
		registers[instr[2]] = argval(instr[2], registers) + argval(instr[3], registers)
	elseif instr[1] == "mul" then
		registers[instr[2]] = argval(instr[2], registers) * argval(instr[3], registers)
	elseif instr[1] == "mod" then
		registers[instr[2]] = argval(instr[2], registers) % argval(instr[3], registers)
	elseif instr[1] == "jgz" then
		if argval(instr[2], registers) > 0 then
			return argval(instr[3], registers)
		end
	else
		print("Invalid instruction", instr[1])
		return nil
	end

	return 1
end

iptr = 0

while true do
	instr = instructions[iptr]

	if instr[1] == "snd" then
		last_played = argval(instr[2], registers)
		iptr = iptr + 1
	elseif instr[1] == "rcv" then
		print("Last played", last_played)
		iptr = iptr + 1
		break
	else
		iptr = iptr + alu(instr, registers)
	end
end

registers = {{p=0}, {p=1}}

iptr = {0, 0} -- instruction pointers
rbuf = {{}, {}} -- receive buffers
rptr = {0, 0} -- Pointer in the receive buffer
rctr = {0, 0} -- Number of entries in each buffer
waiting = {false, false} -- Whether one thread is waiting

pid = 1

function nextpid(pid)
	if pid == 1 then
		return 2
	else
		return 1
	end
end

while true do
	instr = instructions[iptr[pid]]

	if waiting[pid] then
		print("Deadlocked!")
		break
	end

	if instr[1] == "snd" then
		npid = nextpid(pid)
		rbuf[npid][rctr[npid]] = argval(instr[2], registers[pid])
		rctr[npid] = rctr[npid] + 1
		waiting[npid] = false

		iptr[pid] = iptr[pid] + 1
	elseif instr[1] == "rcv" then
		if rbuf[pid][rptr[pid]] ~= nil then
			-- Have value, read and continue
			registers[pid][instr[2]] = rbuf[pid][rptr[pid]]
			rptr[pid] = rptr[pid] + 1
			iptr[pid] = iptr[pid] + 1
		else
			-- Need to wait for value, continue with execution of the other
			waiting[pid] = true
			pid = nextpid(pid)
		end
	else
		iptr[pid] = iptr[pid] + alu(instr, registers[pid])
	end

end
print("Final sends from 1:", rctr[1])
