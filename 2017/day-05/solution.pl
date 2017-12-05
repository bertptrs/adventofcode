#!/usr/bin/perl
my @instructions;

for $line ( <STDIN> ) {
	push @instructions, $line;
}

@instructions2 = @instructions;

$iptr = 0;

$steps = 0;

while ($iptr >= 0 and $iptr < 0 + @instructions) {
	$jump = @instructions[$iptr];
	@instructions[$iptr]++;
	$steps++;

	$iptr += $jump
}

print $steps, "\n";

$iptr = 0;
$steps = 0;

while ($iptr >= 0 and $iptr < 0 + @instructions2) {
	$jump = @instructions2[$iptr];
	if ($jump >= 3) {
		@instructions2[$iptr]--;
	} else {
		@instructions2[$iptr]++;
	}
	$steps++;

	$iptr += $jump
}
print $steps, "\n";
