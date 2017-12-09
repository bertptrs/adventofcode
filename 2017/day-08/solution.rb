#!/usr/bin/env ruby


$registers = Hash.new(0)

def check_oper(a, oper, b)
	a = $registers[a]
	b = b.to_i

	case oper
	when ">"
		return a > b
	when "<"
		return a < b
	when ">="
		return a >= b
	when "<="
		return a <= b
	when "!="
		return a != b
	when "=="
		return a == b
	end
	print oper
end

def do_oper(register, oper, val)
	if oper == "inc"
		$registers[register] += val.to_i
	else
		$registers[register] -= val.to_i
	end
end

def cur_max
	return $registers.max_by {|k,v| v}[1]
end

overall_max = 0

while line = gets
	parts = line.chomp.split " "

	if check_oper parts[4], parts[5], parts[6]
		do_oper parts[0], parts[1], parts[2]
		overall_max = [overall_max, cur_max].max
	end

end

print cur_max, "\n"
print overall_max, "\n"
