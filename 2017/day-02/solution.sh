#!/bin/bash

sum1=0
sum2=0

while read -r line; do
	sorted=$(tr '\t' '\n' <<< "$line" | sort -n)
	((sum1 += ${sorted##*$'\n'} - ${sorted%%$'\n'*}))

	for a in $sorted; do
		for b in $sorted; do
			if [[ $a -le $b ]]; then
				break
			fi

			if [[ $((a % b)) -eq 0 ]]; then
				((sum2 += a / b))
				break
			fi
		done
	done
done

echo Sum 1: $sum1
echo Sum 2: $sum2
