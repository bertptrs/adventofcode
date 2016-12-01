#!/bin/bash

rundir()
{
	input=""
	if [ -f $1/input.txt ]; then
		input="$1/input.txt"
	fi

	for i in $1/*.py; do
		echo "$i" "$input"
		time python "$i" "$input"
		echo
	done
}

for i in day-*; do
	rundir $i
done
