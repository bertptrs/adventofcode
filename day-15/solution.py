from __future__ import print_function
import fileinput
import re
from operator import mul
from functools import reduce

pattern = re.compile(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)")

ingredients = []
calories = []

def computeScore(amounts):
    scores = [sum(ingredients[idx][x] * value for idx, value in enumerate(amounts)) for x in range(4)]

    return reduce(mul, [max(x, 0) for x in scores], 1)

def computeCalories(amounts):
    return sum(a * b for a, b in zip(calories, amounts))


def computeMax(ingredient, budget, currentAmounts):
    budgetLeft = budget - sum(currentAmounts[:ingredient])

    if ingredient == len(ingredients) - 1:
        currentAmounts[ingredient] = budgetLeft
        score = computeScore(currentAmounts)
        if computeCalories(currentAmounts) == 500:
            return score, score

        return score, None

    results = []
    resultsCorrect = [0]
    for n in range(budgetLeft + 1):
        currentAmounts[ingredient] = n
        general, correct = computeMax(ingredient + 1, budget, currentAmounts)
        results.append(general)
        resultsCorrect.append(correct)

    return max(results), max(x for x in resultsCorrect if x is not None)

for line in fileinput.input():
    match = pattern.match(line)
    ingredients.append([int(x) for x in match.group(2, 3, 4, 5)])
    calories.append(int(match.group(6)))

print(computeMax(0, 100, [0] * len(ingredients)))
