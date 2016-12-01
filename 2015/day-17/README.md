# Day 17 efficient solution

Day 17 is an instance of the subset sum problem. This problem asks whether for
a (multi)set of integers *V*, there is a non-empty subset of integers summing up to
exactly *s*. This problem is NP-complete.

The brute force approach of this is trying every possible set in the powerset
of *V* to see if they match. This is inpractical however, because a powerset of
a set of size *n* contains 2<sup>2</sup> sets.

In the exercise, we have 20 buckets, and 2<sup>20</sup> is still
brute-forcable. There is a smarter approach.

We split the list of buckets in two lists of (approximately) the same size. We
then take the powersets of those two lists and compute the sum for each entry.
This leaves us with a total of 2<sup>n / 2 + 1</sup> entries. We then sort both
sublists on the total value of each entry.

Finally, we iterate of the first list, and use binary search to see whether
there is an appropriately sized entry in the second list. This gives us a final
complexity of *n* times 2<sup>*n*/2</sup>, allowing the solution to be computed
instantly.

The algorithm above can be modified to find all combinations, not just one, in
time proportional to the number of solutions. This is implemented in the final
program.
