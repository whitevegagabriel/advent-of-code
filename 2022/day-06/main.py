#!/bin/python3


def is_all_unique(elements: list) -> bool:
    contents = set()
    for item in elements:
        if item in contents:
            return False

        contents.add(item)
    return True


def part_one(input_list: list):
    buffer = input_list[0]
    buffer_pos = 4
    while not is_all_unique(buffer[buffer_pos - 4:buffer_pos]):
        buffer_pos += 1

    print(f'Part one - : {buffer_pos}')


def part_two(input_list: list):
    buffer = input_list[0]
    buffer_pos = 14
    while not is_all_unique(buffer[buffer_pos - 14:buffer_pos]):
        buffer_pos += 1

    print(f'Part two - : {buffer_pos}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
