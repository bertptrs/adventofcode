#!/usr/bin/php
<?php

$maze = file('php://stdin');

$x = strpos($maze[0], '|');
$y = 0;

$dir = [0, 1];

$path = '';
$steps = 0;

while ($maze[$y][$x] != ' ') {
    if (ctype_alpha($maze[$y][$x])) {
        $path .= $maze[$y][$x];
    } elseif ($maze[$y][$x] == '+') {
        if ($dir[0] == 0) {
            $dir[0] = ($x + 1 >= strlen($maze[$y]) || $maze[$y][$x + 1] == ' ') ? -1 : 1;
            $dir[1] = 0;
        } else {
            $dir[0] = 0;
            $dir[1] = ($y + 1 >= count($maze) || $maze[$y + 1][$x] == ' ') ? -1 : 1;
        }
    }

    $x += $dir[0];
    $y += $dir[1];
    $steps++;
}

echo $path . PHP_EOL;
echo $steps . PHP_EOL;
