#!/usr/bin/perl

use strict;
use warnings;

my @instructions;

for my $line ( <STDIN> ) {
	push @instructions, $line;
}

my @instructions2 = @instructions;

my $iptr = 0;

my $steps = 0;

while ($iptr >= 0 and $iptr < @instructions) {
	my $jump = $instructions[$iptr];
	$instructions[$iptr]++;
	$steps++;

	$iptr += $jump
}

print $steps, "\n";

$iptr = 0;
$steps = 0;

while ($iptr >= 0 and $iptr < @instructions2) {
	my $jump = $instructions2[$iptr];
	if ($jump >= 3) {
		$instructions2[$iptr]--;
	} else {
		$instructions2[$iptr]++;
	}
	$steps++;

	$iptr += $jump
}
print $steps, "\n";
