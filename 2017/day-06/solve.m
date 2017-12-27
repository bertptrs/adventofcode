#!/usr/bin/env octave-cli
data = input('', 's');
data = sscanf(data, '%d');

[cycles,state] = solution(data);
cycles
[cycles,~] = solution(state)
