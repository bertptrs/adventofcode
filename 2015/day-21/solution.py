from __future__ import print_function, division
from itertools import product, combinations
import math

def price(c):
    return c[0][0] + c[1][0] + c[2][0][0] + c[2][1][0]

def power(c):
    return c[0][1] + c[1][1] + c[2][0][1] + c[2][1][1]

def armor(c):
    return c[0][2] + c[1][2] + c[2][0][2] + c[2][1][2]

def wins(c, monsterHP, monsterAttack, monsterArmor):
    monsterDamage = max(1, monsterAttack - armor(c))
    heroDamage = max(1, power(c) - monsterArmor)

    return math.ceil(100 / monsterDamage) >= math.ceil(monsterHP / heroDamage)

weapons = [
        (8, 4, 0),
        (10, 5, 0),
        (25, 6, 0),
        (40, 7, 0),
        (74, 8, 0)
        ]

armors = [
        (0, 0, 0), # Dummy armor
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5)
        ]

rings = list(combinations([
        (0, 0, 0), # Dummy ring
        (0, 0, 0), # Second dummy ring
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
        ], 2))


print(min(price(c) for c in product(weapons, armors, rings) if wins(c, 103, 9, 2)))
print(max(price(c) for c in product(weapons, armors, rings) if not wins(c, 103, 9, 2)))
