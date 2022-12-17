#!/bin/python3


def update_max(num_list: list, num: int) -> list:
    if num > num_list[0]:
        num_list[2] = num_list[1]
        num_list[1] = num_list[0]
        num_list[0] = num
    elif num > num_list[1]:
        num_list[2] = num_list[1]
        num_list[1] = num
    elif num > num_list[2]:
        num_list[2] = num

    return num_list


def part_one(calories_list: list):
    partial_sum = 0
    max_sum = 0

    for calorie in calories_list:
        if calorie == "":
            max_sum = max(partial_sum, max_sum)
            partial_sum = 0
        else:
            partial_sum += int(calorie)

    print(f"Part one - max calories: {max_sum}")


def part_two(calories_list: list):
    partial_sum = 0
    max_sums = [0, 0, 0]

    for calorie in calories_list:
        if calorie == "":
            max_sums = update_max(max_sums, partial_sum)
            partial_sum = 0
        else:
            partial_sum += int(calorie)

    max_sum = sum(max_sums)

    print(f"Part two - top three max sum: {max_sum}")


if __name__ == "__main__":
    contents_list = open("input.txt").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
