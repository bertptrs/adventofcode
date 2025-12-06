#!/usr/bin/env php
<?php

if ($argc < 2) {
    echo "Usage: {$argv[0]} INPUT\n";
    exit(1);
}

$input = file($argv[1]);

function part1(string ...$input): int {
    $lines = [];

    foreach ($input as $line) {
        $lines[] = preg_split('/\s+/', $line, -1, PREG_SPLIT_NO_EMPTY);
    }

    $lines = array_reverse($lines);
    $cols = count($lines[0]);
    $rows = count($lines);

    $sum = 0;

    for ($col = 0; $col < $cols; ++$col) {
        if ($lines[0][$col] === '+') {
            $acc = 0;
            for ($row  = 1; $row < $rows; ++$row) {
                $acc += (int) $lines[$row][$col];
            }
        } else {
            $acc = 1;
            for ($row  = 1; $row < $rows; ++$row) {
                $acc *= (int) $lines[$row][$col];
            }
        }

        $sum += $acc;
    }

    return $sum;
}

function part2(string ...$lines): int {
    $cols = strlen($lines[0]);
    $rows = count($lines);
    $numbers = array_fill(0, $cols, "");

    for ($row = 0; $row < $rows - 1; ++$row) {
        foreach (str_split($lines[$row]) as $col => $c) {
            if ($c !== ' ') {
                $numbers[$col] .= $c;
            }
        }
    }

    $sum = 0;

    foreach (str_split($lines[$rows - 1]) as $col => $c) {
        switch ($c) {
            case '+':
                $acc = 0;
                for ($i = $col; $i < $cols && $numbers[$i] !== ""; ++$i) {
                    $acc += (int) $numbers[$i];
                }
                $sum += $acc;
                break;
            case '*':
                $acc = 1;
                for ($i = $col; $i < $cols && $numbers[$i] !== ""; ++$i) {
                    $acc *= (int) $numbers[$i];
                }
                $sum += $acc;
                break;
        }
    }

    return $sum;
}

echo "Part1: " . part1(...$input) . "\n";
echo "Part2: " . part2(...$input) . "\n";
