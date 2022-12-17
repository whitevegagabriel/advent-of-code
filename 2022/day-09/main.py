#!/bin/python3


def create_grid_for_size(moves):
    x_max = 0
    x_min = 0
    y_max = 0
    y_min = 0
    x = 0
    y = 0
    for move in moves:
        move = move.split()
        if move[0] == "R":
            x += int(move[1])
            x_max = max(x, x_max)
        elif move[0] == "L":
            x -= int(move[1])
            x_min = min(x, x_min)
        elif move[0] == "U":
            y += int(move[1])
            y_max = max(y, y_max)
        elif move[0] == "D":
            y -= int(move[1])
            y_min = min(y, y_min)
        else:
            print("panic")

    return [[0] * (y_max - y_min + 1)
            for i in range((x_max - x_min + 1))], [-x_min, -y_min]


def move_tail(tail_pos, head_pos):
    new_tail = tail_pos[:]

    if tail_pos[0] + 2 == head_pos[0] and tail_pos[1] + 2 == head_pos[1]:
        new_tail[0] += 1
        new_tail[1] += 1
    elif tail_pos[0] + 2 == head_pos[0] and tail_pos[1] - 2 == head_pos[1]:
        new_tail[0] += 1
        new_tail[1] -= 1
    elif tail_pos[0] - 2 == head_pos[0] and tail_pos[1] + 2 == head_pos[1]:
        new_tail[0] -= 1
        new_tail[1] += 1
    elif tail_pos[0] - 2 == head_pos[0] and tail_pos[1] - 2 == head_pos[1]:
        new_tail[0] -= 1
        new_tail[1] -= 1
    elif tail_pos[0] + 2 == head_pos[0]:
        new_tail[0] += 1
        new_tail[1] = head_pos[1]
    elif tail_pos[0] - 2 == head_pos[0]:
        new_tail[0] -= 1
        new_tail[1] = head_pos[1]
    elif tail_pos[1] + 2 == head_pos[1]:
        new_tail[1] += 1
        new_tail[0] = head_pos[0]
    elif tail_pos[1] - 2 == head_pos[1]:
        new_tail[1] -= 1
        new_tail[0] = head_pos[0]
    return new_tail


def part_one(input_list):
    visited_grid, head_pos = create_grid_for_size(input_list)
    tail_pos = head_pos[:]

    for item in input_list:
        move = item.split()
        for i in range(int(move[1])):
            if move[0] == "R":
                head_pos[0] += 1
            elif move[0] == "L":
                head_pos[0] -= 1
            elif move[0] == "U":
                head_pos[1] += 1
            elif move[0] == "D":
                head_pos[1] -= 1
            else:
                print("panic")
                exit()
            tail_pos = move_tail(tail_pos, head_pos)
            visited_grid[tail_pos[0]][tail_pos[1]] = 1

    visited_places = sum([sum(row) for row in visited_grid])
    print(f'Part one - : {visited_places}')


def part_two(input_list):
    visited_grid, head_pos = create_grid_for_size(input_list)
    rope = [head_pos[:] for i in range(10)]

    for item in input_list:
        move = item.split()
        for i in range(int(move[1])):
            if move[0] == "R":
                rope[0][0] += 1
            elif move[0] == "L":
                rope[0][0] -= 1
            elif move[0] == "U":
                rope[0][1] += 1
            elif move[0] == "D":
                rope[0][1] -= 1
            else:
                print("panic")
                exit()
            for i in range(1, len(rope)):
                rope[i] = move_tail(rope[i], rope[i - 1])
            visited_grid[rope[9][0]][rope[9][1]] = 1

    visited_places = sum([sum(row) for row in visited_grid])
    print(f'Part two - : {visited_places}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
