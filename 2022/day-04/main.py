#!/bin/python3
import re


def is_overlapping(first_pair: list[int], second_pair: list[int]) -> bool:
    return not (first_pair[0] > second_pair[1] or second_pair[0] > first_pair[1])


def is_fully_overlapping(first_pair: list[int], second_pair: list[int]) -> bool:
    return (first_pair[0] <= second_pair[0] and first_pair[1] >= second_pair[1]
            or second_pair[0] <= first_pair[0] and second_pair[1] >= first_pair[1])


def part_one(input_list: list):
    assignment_pairs = [re.split(r"[^\d]", pair) for pair in input_list]
    fully_overlapping_count = 0
    for assignment_pair in assignment_pairs:
        assignment_pair_int = [int(item) for item in assignment_pair]

        first_pair = assignment_pair_int[0:2]
        second_pair = assignment_pair_int[2:4]

        if is_fully_overlapping(first_pair, second_pair):
            fully_overlapping_count += 1

    print(f"Part one - : {fully_overlapping_count}")


def part_two(input_list: list):
    assignment_pairs = [re.split(r"[^\d]", pair) for pair in input_list]
    overlapping_count = 0
    for assignment_pair in assignment_pairs:
        assignment_pair_int = [int(item) for item in assignment_pair]

        first_pair = assignment_pair_int[0:2]
        second_pair = assignment_pair_int[2:4]

        if is_overlapping(first_pair, second_pair):
            overlapping_count += 1

    print(f"Part two - : {overlapping_count}")


if __name__ == "__main__":
    contents_list = open("input.txt").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
