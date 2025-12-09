#!/usr/bin/env ts-node
import * as fs from 'fs';
import { exit } from 'process';

if (process.argv.length < 3) {
    console.log("Usage: " + process.argv0 + " " + process.argv[1] + " INPUT_FILE");
    exit(10);
}

const input_file = fs.readFileSync(process.argv[2], "utf-8");
const lines = input_file.trim().split("\n");

class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }
}

const points = lines.map(line => {
    const [x, y] = line.split(",");
    return new Point(+x, +y);
});

let max_size = 0;

for (let i = 0; i < points.length; ++i) {
    for (let j = i + 1; j < points.length; ++j) {
        const width = Math.abs(points[i].x - points[j].x) + 1;
        const height = Math.abs(points[i].y - points[j].y) + 1;

        max_size = Math.max(max_size, width * height);
    }
}

console.log("Part 1:", max_size);
