#!/bin/bash

run () {
	pushd "$1" &> /dev/null || exit 1

	local epilogue=""
	if [[ -f input.txt ]]; then
		epilogue=" < input.txt"
	fi

	local prog="make"
	if [[ ! -f Makefile ]]; then
		prog=$(find . -type f -executable)
		if [[ -z "$prog" ]] || [[ "$(wc -l <<< "$prog")" -gt 1 ]]; then
			echo "No executable"
			exit 1
		fi
	fi

	local cmd="time $prog $epilogue"

	eval "$cmd"

	popd &> /dev/null || exit 1
}

for day in day-*; do
	echo "$day" "$(run "$day" |& grep -i "real" | awk '{print $2}')"
done
