#!/bin/bash

rundir()
{
	if hash pypy &> /dev/null; then
		python=pypy
	else
		python=python
	fi

	input=""
	if [ -f $1/input.txt ]; then
		input="$1/input.txt"
	fi

	for i in $1/*.py; do
		echo "$i" "$input"
		$python "$i" "$input"
	done
}

for i in day-*; do
	rundir $i
done
