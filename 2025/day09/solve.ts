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
let max_size_contained = 0;

function has_intersection(left: number, right: number, bottom: number, top: number): boolean {
    for (let i = 0; i < points.length; ++i) {
        const first = points[i];
        const second = points[(i + 1) % points.length];

        if (first.x == second.x) {
            const yMin = Math.min(first.y, second.y);
            const yMax = Math.max(first.y, second.y);

            if (left < first.x && first.x < right && (yMin <= bottom && bottom < yMax || yMin < top && top <= yMax)) {
                return true;
            }
        } else if (first.y == second.y) {
            const xMin = Math.min(first.x, second.x);
            const xMax = Math.max(first.x, second.x);

            if (bottom < first.y && first.y < top && (xMin <= left && left < xMax || xMin < right && right <= xMax)) {
                return true;
            }
        } else {
            throw "Invalid input";
        }
    }
    return false;
}

for (let i = 0; i < points.length; ++i) {
    for (let j = i + 1; j < points.length; ++j) {
        const left = Math.min(points[i].x, points[j].x);
        const right = Math.max(points[i].x, points[j].x);
        const bottom = Math.min(points[i].y, points[j].y);
        const top = Math.max(points[i].y, points[j].y);

        const width = right - left + 1;
        const height = top - bottom + 1;

        const area = width * height;

        max_size = Math.max(max_size, area);
        if (area > max_size_contained && !has_intersection(left, right, bottom, top)) {
            max_size_contained = area;
        }
    }
}

console.log("Part 1:", max_size);
// Too high: 4531758980
console.log("Part 2:", max_size_contained);
