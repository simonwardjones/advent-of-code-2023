import time
from functools import cache
from pathlib import Path


def part_two():
    data = tuple(
        tuple(frozenset(part.split()) for part in line.split(": ")[1].split(" | "))
        for line in Path("input/day_04.txt").read_text().strip().split("\n")
    )
    return sum(card_wins(i, data) for i in range(len(data)))


@cache
def card_wins(id, cards):
    matched = len(cards[id][0].intersection(cards[id][1]))
    return 1 + sum(card_wins(id + i, cards) for i in range(1, matched + 1))


def main():
    answer = part_two()
    print(f"{answer=}")


start_time = time.time()
main()
print("--- %s seconds ---" % (time.time() - start_time))
