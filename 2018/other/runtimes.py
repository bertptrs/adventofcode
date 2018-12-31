#!/usr/bin/env python3
import fileinput
import re
import numpy as np
import matplotlib.pyplot as plt


def parse_time(val):
    val = val.replace(",", "")
    return int(val) / 1e9


def parse_input():
    matcher = re.compile(r"day(\d{2})::part(1|2) ... bench:(\s+)([0-9,]+) ns/iter \(\+/- ([0-9,]+)\)$")
    results = np.zeros((25, 2, 2))
    for line in sorted(fileinput.input()):
        match = matcher.search(line)
        assert match

        day = int(match.group(1)) - 1
        part = int(match.group(2)) - 1
        time = parse_time(match.group(4))
        variance = parse_time(match.group(5))
        results[day, part, :] = [time, variance]

    return results


def main():
    results = parse_input()
    _, ax = plt.subplots()
    ax.set_yscale('log')
    ax.set_xlabel('Day')
    ax.set_ylabel('Runtime (s)')
    ax.set_xticks(list(range(1, 26)))

    bottom = np.min(results)

    for i in range(results.shape[1]):
        values = results[:, i, 0]
        errors = results[:, i, 1]
        pos = np.arange(len(values)) + 0.8 + 0.4 * i
        ax.bar(pos, values, yerr=errors, align='center', width=0.4,
               bottom=bottom)

    ax.legend(['Part 1', 'Part 2'])

    plt.savefig('runtime.svg')


if __name__ == '__main__':
    main()
