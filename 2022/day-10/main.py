#!/bin/python3


def part_one(input_list):
    registers = []
    value = 1

    for instruction in input_list:
        instruction = instruction.split()
        registers.append(value)
        if instruction[0] == "addx":
            registers.append(value)
            value += int(instruction[1])

    cycles = [20, 60, 100, 140, 180, 220]

    signal_sum = sum([cycle * registers[cycle - 1] for cycle in cycles])

    print(f'Part one - : {signal_sum}')


def part_two(input_list):
    registers = []
    value = 1

    for instruction in input_list:
        instruction = instruction.split()
        registers.append(value)
        if instruction[0] == "addx":
            registers.append(value)
            value += int(instruction[1])

    num_rows = 6
    num_cols = 40
    screen_buffer = [["."] * num_cols for i in range(num_rows)]
    for index, sprite_pos in enumerate(registers):
        row = index // num_cols
        col = index % num_cols
        if abs(sprite_pos - col) <= 1:
            screen_buffer[row][col] = "#"

    screen_buffer_rendered = "\n".join(
        ["".join(line) for line in screen_buffer])
    print('Part two - :')
    print(screen_buffer_rendered)


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
