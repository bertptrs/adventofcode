from __future__ import print_function

def findFirst(data, target):
    return next(idx for idx, value in enumerate(data) if value >= target)

target = 34000000

# Target is achieved at itself/10, so reasonable upper bound.
upperbound = target // 10

# Use a varation of Erathostenes' sieve to compute the results
sieve1 = [10] * (upperbound + 1)
sieve2 = [10] * (upperbound + 1)

for x in range(1, upperbound):
    for y in range(x, upperbound, x):
        sieve1[y] += 10 * x

    for y in range(x, min(50 * x, upperbound) + 1, x):
        sieve2[y] += 11 * x

print("House", findFirst(sieve1, target))
print("House", findFirst(sieve2, target))
