#!/bin/bash

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <input file>" >&2
    exit 1
fi

joltage() {
    local first second c i

    first=0
    second=0

    for ((i = 0; i < ${#1}; i++)); do
        c="${1:$i:1}"

        if [[ $second -gt $first ]]; then
            first="$second"
            second="$c"
        elif [[ $c -gt $second ]]; then
            second="$c"
        fi
    done

    echo "$first$second"
}

joltage2() {
    local c i d e n digits

    digits=(0 0 0 0 0 0 0 0 0 0 0 0)

    for ((i = 0; i < ${#1}; i++)); do
        c="${1:$i:1}"

        for ((d = 0; d < 11; d++)); do
            if [[ ${digits[((d + 1))]} -gt ${digits[$d]} ]]; then
                for ((e = d; e < 11; e++)); do
                    n=$((e + 1))
                    digits[e]=${digits[n]}
                done
                digits[11]="0"
                break
            fi
        done

        if [[ $c -gt ${digits[11]} ]]; then
            digits[11]="$c"
        fi
    done

    printf "%s" "${digits[@]}"
}

total=0
total2=0

while IFS="" read -r line || [[ -n "$p" ]]; do
    ((total += $(joltage "$line")))
    ((total2 += $(joltage2 "$line")))
done <"$1"

echo "$total"
echo "$total2"
