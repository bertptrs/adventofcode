#!/bin/bash

sum1=0
sum2=0

while read -r line; do
	sorted=$(echo "$line" | xargs -n 1 echo | sort -n)
	checksum=$(($(echo "$sorted" | tail -n 1) - $(echo "$sorted" | head -n 1)))

	for a in $sorted; do
		for b in $sorted; do
			if [[ $a -le $b ]]; then
				continue
			fi

			if [[ $((a % b)) -eq 0 ]]; then
				((sum2 += a / b))
				break
			fi
		done
	done

	((sum1 += checksum))
done

echo Sum 1: $sum1
echo Sum 2: $sum2
