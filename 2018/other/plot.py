#!/usr/bin/env python3
'''
Plotter for the length of my solutions, by day.

This excludes common code.
'''
import argparse
import csv
import io
import re
import subprocess
import matplotlib.pyplot as plt
import numpy as np


def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('source', help='Source directory', nargs='?',
                        default='src')
    parser.add_argument('-o', '--output', help='Output for the plot')

    return parser.parse_args()


def cloc_usage(src_dir):
    result = subprocess.run(['cloc', '--by-file', '--csv', '--quiet', src_dir],
                            check=True, capture_output=True)

    output = result.stdout.decode('utf-8')
    data = io.StringIO(output, newline='')

    reader = csv.reader(data, dialect='unix')
    days = []
    values = []

    for line in reader:
        if not line:
            continue

        match = re.search(r"\d+", line[1])
        if match:
            day = int(match.group(0))
            days.append(day)
            values.append(list(int(x) for x in line[2:]))

    data = list(zip(days, values))
    data.sort()

    days, values = list(zip(*data))
    values = np.array(values)

    return days, values


def main():
    args = get_args()
    days, values = cloc_usage(args.source)

    for sequence in values.transpose():
        plt.plot(days, sequence)

    plt.xlabel('Day')
    plt.ylabel('Lines of code')
    plt.legend(['Blank', 'Comment', 'Code'])

    if args.output:
        plt.savefig(args.output)
    else:
        plt.show()


if __name__ == '__main__':
    main()
