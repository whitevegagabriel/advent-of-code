#!/bin/python3


def build_set(collection: list) -> set:
    collection_set = set()
    for item in collection:
        if item not in collection_set:
            collection_set.add(item)

    return collection_set


def get_priority(character: str) -> int:
    zero = ord("`")
    capital_offset = ord("z") - ord("A") + 1
    if character.islower():
        return ord(character) - zero
    else:
        return ord(character) + capital_offset - zero


def find_priority_in_sack(sack: str) -> int:
    first_letters = build_set(sack[:len(sack) // 2])
    for letter in sack[len(sack) // 2:]:
        if letter in first_letters:
            return get_priority(letter)


def find_priority_in_group(group: list) -> int:
    first_letters = build_set(group[0])
    second_letters = build_set(group[1])

    for letter in group[2]:
        if letter in first_letters and letter in second_letters:
            return get_priority(letter)


def part_one():
    rucksacks = open("input.txt").readlines()
    priorities_sum = 0
    for sack in rucksacks:
        priorities_sum += find_priority_in_sack(sack)
    print(f"Part one - sum of priorities: {priorities_sum}")


def part_two():
    rucksacks = open("input.txt").readlines()
    priorities_sum = 0
    for i in range(len(rucksacks) // 3):
        start_index = i * 3
        priorities_sum += find_priority_in_group(
            rucksacks[start_index:start_index + 3])
    print(f"Part two - sum of priorities: {priorities_sum}")


if __name__ == "__main__":
    part_one()
    part_two()
