#!/usr/bin/env python3
import json
from pathlib import Path
from typing import Dict

import numpy as np
import matplotlib.pyplot as plt


def read_timings() -> Dict[int, Dict]:
    timings = {}

    for day in Path('target/criterion/part1').iterdir():
        with open(day / 'new' / 'estimates.json', mode='rb') as f:
            timings[int(day.parts[-1])] = {
                1: json.load(f)
            }

    for day in Path('target/criterion/part2').iterdir():
        with open(day / 'new' / 'estimates.json', mode='rb') as f:
            timings[int(day.parts[-1])][2] = json.load(f)

    return timings


def plot_cumulative_time(timings: Dict[int, Dict]):
    plt.clf()

    times = [0]

    for day in range(min(timings.keys()), max(timings.keys()) + 1):
        times.append(timings[day][1]['mean']['point_estimate'])
        if day < 25:
            times.append(timings[day][2]['mean']['point_estimate'])
        else:
            times.append(0)

    cumulative = np.cumsum(times)
    # Convert from nanoseconds to seconds
    cumulative /= 1e9

    x = np.arange(0.0, 25.5, 0.5)

    plt.plot(x, cumulative, label="Cumulative time", drawstyle='steps-post')
    plt.plot([0, 25], [0, 0.5], label="Target time")
    plt.ylabel('Cumulative time (s)')
    plt.xlabel('Days completed')

    plt.legend()
    plt.tight_layout()

    plt.xlim(0, 25)
    plt.ylim(0, 0.5)

    plt.savefig('cumulative-time.svg')


def plot_individual_times(timings: Dict[int, Dict]):
    plt.clf()

    def plot(parts, **kwargs):
        x = np.arange(1, len(parts) + 1)

        values = np.array(list(part['mean']['point_estimate'] for part in parts))
        upper = np.array(list(part['mean']['confidence_interval']['upper_bound'] for part in parts))
        lower = np.array(list(part['mean']['confidence_interval']['lower_bound'] for part in parts))

        # Convert from ns to s
        yerr = np.array([upper - values, lower - values]) / 1e9
        values = values / 1e9

        plt.bar(x, values, yerr=yerr, align='edge', log=True, **kwargs)
        pass

    plot(list(timings[day][1] for day in range(1, 26)), label="Part 1", width=-0.4)
    plot(list(timings[day][2] for day in range(1, 25)), label="Part 2", width=0.4)

    plt.ylabel('Runtime (s)')
    plt.xlabel('Day')

    plt.xlim(0, 26)
    plt.xticks(np.arange(1, 26))

    plt.legend()
    plt.tight_layout()

    plt.savefig('individual-time.svg')


def main():
    timings = read_timings()
    plot_cumulative_time(timings)
    plot_individual_times(timings)


if __name__ == '__main__':
    main()
