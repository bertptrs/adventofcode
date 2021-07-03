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


def part1(data: TextIO):
    instructions = [line.strip() for line in data]

    result = shuffle(instructions, 10007)

    for i, card in enumerate(result):
        if card == 2019:
            return i

    raise Exception("Did not find card")
