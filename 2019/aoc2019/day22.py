from typing import List, TextIO


def shuffle(instructions: List[str], deck_size: int) -> List[int]:
    deck = list(range(0, deck_size))

    for instruction in instructions:
        if "new stack" in instruction:
            deck = list(reversed(deck))
            continue

        parts = instruction.split(" ")
        if parts[0] == "cut":
            by = int(parts[1])

            new_deck = deck[by:]
            new_deck += deck[:by]

            deck = new_deck
        else:
            increment = int(parts[3])

            new_deck = list(range(0, deck_size))
            target_index = 0

            for card in deck:
                new_deck[target_index] = card
                target_index = (target_index + increment) % len(deck)

            deck = new_deck

    return deck


def part1(data: TextIO) -> int:
    instructions = [line.strip() for line in data]

    result = shuffle(instructions, 10007)

    for i, card in enumerate(result):
        if card == 2019:
            return i

    raise Exception("Did not find card")


def modpow(a: int, b: int, m: int) -> int:
    assert b >= 0

    result = 1
    n = a

    while b > 0:
        if b % 2:
            result = (result * n) % m

        b //= 2
        n = (n * n) % m

    return result


def inverse(a: int, m: int) -> int:
    """ Computes the modulo multiplicative inverse """
    return modpow(a, m - 2, m)


def part2(data: TextIO) -> int:
    deck_size = 119315717514047
    shuffles = 101741582076661

    a, b = 1, 0

    for line in data:
        parts = line.split(' ')
        if 'new stack' in line:
            la, lb = -1, -1
        elif parts[0] == 'deal':
            la, lb = int(parts[-1]), 0
        else:
            la, lb = 1, -int(parts[-1])

        a = (la * a) % deck_size
        b = (la * b + lb) % deck_size

    final_a = modpow(a, shuffles, deck_size)
    final_b = ((b * (final_a - 1)) * inverse(a - 1, deck_size)) % deck_size

    return ((2020 - final_b) * inverse(final_a, deck_size)) % deck_size

