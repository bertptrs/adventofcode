# Usage

Since this solution is in matlab, it works a bit differently. You can
solve part 1 by calling `solution([original division of wealth])`, which
returns the number of cycles and the point at which it repeated. You can
then call `solution` again with that repetition point to get the length
of that cycle.
