#!/bin/python3


def draw_line(start, end, grid):
    if start[0] != end[0]:
        step = (end[0] - start[0]) // abs(end[0] - start[0])
        for i in range(start[0], end[0] + step, step):
            grid[i][start[1]] = "#"
    else:
        step = (end[1] - start[1]) // abs(end[1] - start[1])
        for j in range(start[1], end[1] + step, step):
            grid[start[0]][j] = "#"


def create_cave(input_list):
    max_y = 0
    max_x = 0
    min_y = None
    cave_directions = [directions.split(" -> ") for directions in input_list]
    for i, directions in enumerate(cave_directions):
        cave_directions[i] = [coord.split(",") for coord in directions]
        for coord in cave_directions[i]:
            coord.reverse()
            coord[0] = int(coord[0])
            coord[1] = int(coord[1])
            if min_y == None:
                min_y = coord[1]
            else:
                min_y = min(min_y, coord[1])
            max_x = max(max_x, coord[0])
            max_y = max(max_y, coord[1])

    min_y = min(min_y, 500 - max_x - 2)
    max_y = max(max_y, 500 + max_x + 2)
    for directions in cave_directions:
        for coord in directions:
            coord[1] -= min_y

    y_range = max_y - min_y + 1
    x_range = max_x + 3

    cave = [["."] * y_range for i in range(x_range)]

    for directions in cave_directions:
        start = directions[0]
        for coord in directions[1:]:
            draw_line(start, coord, cave)
            start = coord

    return cave, 500 - min_y


def sand_has_settled(cave, sand_coord):
    if cave[0][sand_coord] != ".":
        return False

    sand = [0, sand_coord]

    sand_has_settled = None
    while sand_has_settled == None:
        if sand[0] + 1 == len(cave):
            sand_has_settled = False
            break

        if cave[sand[0] + 1][sand[1]] == ".":
            sand[0] += 1
        elif sand[1] > 0 and cave[sand[0] + 1][sand[1] - 1] == ".":
            sand[0] += 1
            sand[1] -= 1
        elif sand[1] + 1 < len(cave[0]) and cave[sand[0] + 1][sand[1] +
                                                              1] == ".":
            sand[0] += 1
            sand[1] += 1
        elif sand[1] + 1 == len(cave[0]) or sand[1] == 0:
            sand_has_settled = False
        else:
            sand_has_settled = True
            cave[sand[0]][sand[1]] = "O"

    return sand_has_settled


def part_one(input_list):
    cave, sand_coord = create_cave(input_list)

    dropped_sand = 0
    while sand_has_settled(cave, sand_coord):
        dropped_sand += 1

    print(f'Part one - : {dropped_sand}')


def part_two(input_list):
    cave, sand_coord = create_cave(input_list)
    draw_line([len(cave) - 1, 0], [len(cave) - 1, len(cave[0]) - 1], cave)

    dropped_sand = 0
    while sand_has_settled(cave, sand_coord):
        dropped_sand += 1

    print(f'Part two - : {dropped_sand}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
