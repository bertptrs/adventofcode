from __future__ import print_function, division
from queue import PriorityQueue

def applyEffects(current):
    if current['recharge'] > 0:
        current['mana'] += 101
        current['recharge'] -= 1

    if current['shield'] > 0:
        current['shield'] -= 1

    if current['poison'] > 0:
        current['poison'] -= 1
        current['bossHP'] -= 3


def dijkstraMagic(initialState, hard):
    todo = PriorityQueue()
    todo.put((0, 0, initialState.copy()))

    visited = {tuple(initialState.values()): 0}
    iteration = 0

    while not todo.empty():
        iteration += 1
        top = todo.get()
        current = top[2].copy()
        manaSpent = top[0]

        if hard:
            # Hard mode
            current['hp'] -= 1

        if current['hp'] <= 0:
            # Already dead
            continue

        applyEffects(current)

        if current['bossHP'] <= 0:
            # We won!
            return manaSpent

        # Try every available spell
        for spell in spells:
            if current['mana'] < spell[1]:
                # Not enough mana to cast
                continue

            if spell[4] is not None and current[spell[4]] > 0:
                # Effect still active
                continue

            newState = current.copy()
            newState['bossHP'] -= spell[2]
            newState['mana'] -= spell[1]
            newState['hp'] += spell[3]
            if spell[4] is not None:
                newState[spell[4]] = spell[5]

            newMana = manaSpent + spell[1]

            # Start Boss turn
            applyEffects(newState)

            if newState['bossHP'] > 0:
                # Alive to attack?
                if newState['shield'] > 0:
                    damage = 2
                else:
                    damage = 9

                newState['hp'] -= damage
            else:
                # Died during effects
                return newMana

            if newState['hp'] > 0:
                # New state
                key = tuple(newState.values())
                if key not in visited or visited[key] > newMana:
                    # Insert the route in to the visited and queue
                    visited[key] = newMana
                    todo.put((newMana, iteration, newState))

    return None

spells = [
        # Name, Mana, Damage, Heal, Effect, Turns
        ("Magic Missile", 53, 4, 0, None, 0),
        ("Drain", 73, 2, 2, None, 0),
        ("Shield", 113, 0, 0, "shield", 6),
        ("Poison", 173, 0, 0, "poison", 6),
        ("Recharge", 229, 0, 0, "recharge", 5),
        ]

status = {
        "bossHP": 51,
        "hp": 50,
        "mana": 500,
        "shield": 0,
        "poison": 0,
        "recharge": 0
        }

print(dijkstraMagic(status, False))
print(dijkstraMagic(status, True))
