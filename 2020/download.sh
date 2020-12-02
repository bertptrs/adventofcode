#!/bin/bash

if [[ -z $AOC_SESSION ]]; then
	echo "AOC_SESSION not set"
	exit 1
fi

if [[ $# -ge 1 ]]; then
	# Get rid of leading zeroes
	(( DAY = $1 * 1 ))
else
	DAY=$(date +%-d)
fi

TARGET_FILE=$(printf "inputs/%02d.txt" "$DAY")

echo "Going to download day $DAY to $TARGET_FILE"

if [[ -s $TARGET_FILE ]]; then
	echo "Target file exists"
else
	curl --output "$TARGET_FILE" \
		--header "Cookie: session=$AOC_SESSION" \
		--fail \
		"https://adventofcode.com/2020/day/$DAY/input"
fi
