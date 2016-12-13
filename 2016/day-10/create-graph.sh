#!/bin/bash

echo "digraph bots {"

while IFS=' ' read -r -a data; do
	if [[ ${data[0]} == "value" ]]; then
		echo "I${data[1]} -> B${data[5]};"
	else
		[[ ${data[5]} == "bot" ]] && kind1="B" || kind2="O"
		[[ ${data[10]} == "bot" ]] && kind2="B" || kind2="O"
		echo "B${data[1]} -> $kind1${data[6]} [label=lo];"
		echo "B${data[1]} -> $kind2${data[11]} [label=hi];"
	fi
done
echo "}"
