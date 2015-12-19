# Day 6 efficient solution

For day 6, we have an array of lights (first booleans, then integers) on which
we want to perform some tasks, and then know the end result. To do so, the
easiest method is to simulate that process.

All operations execute on a specific rectangle of lights. This means that we
need to modify a potentially large number of lights at the same time. This is a
rather costly operation.

To get around this, we use run length encoding. Instead of storing a boolean
for each light, we store, per column, a list of runs of lights that are in the
same state. That way, the complexity of lights to flip is now proportional to
the number of rows we need to flip.

## Updating the runs

When we need to update the runs for a given column, we iterate over all runs
that are in that column. If a run is within the row range that we need to
modify, we can set the state of the run to the proper value. You may need to
split the run in several parts that are not all affected by the modification.

As a second step, we try to combine adjacent runs that are in the same state.
This reduces the cost of iterating over all of them significantly.

## Performance analysis

On my computer, the naive brute force implementation took about 5 seconds for
both part 1 and part 2. When running the run length implementation on the first
part, the algorithm is 20 times faster. However, on the second part, the
algorithm is only slightly faster (3.5 seconds) because runs can have more
values, and are thus not as easily combined.
