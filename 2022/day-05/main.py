#!/bin/python3
import re


def get_base_index(input_list: list) -> int:
    base_index = 0
    base_index_found = False
    while not base_index_found:
        if input_list[base_index][1] == "1":
            base_index_found = True
        else:
            base_index += 1
    return base_index


def get_stacks_as_list(input_list: list) -> list:
    base_index = get_base_index(input_list)

    max_cols = int(max(input_list[base_index].split()))

    # initialize stacks to correct number of columns
    stacks_list = [[] for i in range(max_cols)]
    # isolate the stacks and reverse to traverse bottom->top
    for row in reversed(input_list[:base_index]):
        # normalize the row
        row = row[1:]

        # find the ith letter in a row and append to the stacks if not a blank space
        step_size = 4
        for i in range(max_cols):
            potential_letter = row[i * step_size]
            if potential_letter != " ":
                stacks_list[i].append(potential_letter)
    return stacks_list


def get_moves_as_list(input_list: list) -> list:
    base_index = get_base_index(input_list)

    moves_list = []
    for move in input_list[base_index + 2:]:
        move = move.replace("move ", "").replace(" from ",
                                                 " ").replace(" to ",
                                                              " ").split()
        moves_list.append(move)
    return moves_list


def part_one(input_list: list):
    stacks_list = get_stacks_as_list(input_list)
    moves_list = get_moves_as_list(input_list)

    for move in moves_list:
        move_quant = int(move[0])
        from_stack = int(move[1]) - 1
        to_stack = int(move[2]) - 1

        for _ in range(move_quant):
            letter = stacks_list[from_stack].pop()
            stacks_list[to_stack].append(letter)

    top_of_stacks = [stack[-1] for stack in stacks_list]

    print(f'Part one - : {"".join(top_of_stacks)}')


def part_two(input_list: list):
    stacks_list = get_stacks_as_list(input_list)
    moves_list = get_moves_as_list(input_list)

    for move in moves_list:
        move_quant = int(move[0])
        from_stack = int(move[1]) - 1
        to_stack = int(move[2]) - 1

        letters = stacks_list[from_stack][-move_quant:]
        stacks_list[from_stack] = stacks_list[from_stack][:-move_quant]
        stacks_list[to_stack] += letters

    top_of_stacks = [stack[-1] for stack in stacks_list]

    print(f'Part two - : {"".join(top_of_stacks)}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
