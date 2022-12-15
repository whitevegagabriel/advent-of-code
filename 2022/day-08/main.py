#!/bin/python3


def visible_from_less_x(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for i in range(0, x):
        if tree_grid[i][y] >= tree_height:
            return False

    return True

def visible_from_more_x(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for i in range(x+1, len(tree_grid)):
        if tree_grid[i][y] >= tree_height:
            return False

    return True


def visible_from_more_y(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for j in range(y+1, len(tree_grid[0])):
        if tree_grid[x][j] >= tree_height:
            return False

    return True


def visible_from_less_y(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for j in range(0, y):
        if tree_grid[x][j] >= tree_height:
            return False

    return True


def count_visible_less_x(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for i in range(x-1, -1, -1):
        if tree_grid[i][y] >= tree_height:
            return x - i

    return x

def count_visible_more_x(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for i in range(x+1, len(tree_grid)):
        if tree_grid[i][y] >= tree_height:
            return i - x

    return len(tree_grid) - x - 1


def count_visible_more_y(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for j in range(y+1, len(tree_grid[0])):
        if tree_grid[x][j] >= tree_height:
            return j - y

    return len(tree_grid[0]) - y - 1


def count_visible_less_y(tree_grid, x, y):
    tree_height = tree_grid[x][y]

    for j in range(y-1, -1, -1):
        if tree_grid[x][j] >= tree_height:
            return y - j

    return y


def part_one(input_list):
    visible_trees = (len(input_list) + len(input_list[0]) - 2) * 2

    for x in range(1, len(input_list)-1):
        for y in range(1, len(input_list[0])-1):
            if (visible_from_less_x(input_list, x, y)
                or visible_from_more_x(input_list, x, y)
                or visible_from_more_y(input_list, x, y)
                or visible_from_less_y(input_list, x, y)):
                visible_trees += 1

    print(f'Part one - : {visible_trees}')


def part_two(input_list):
    max_scenic_score = 0

    for x in range(1, len(input_list)-1):
        for y in range(1, len(input_list[0])-1):
            scenic_score = count_visible_less_x(input_list, x, y) \
                * count_visible_more_x(input_list, x, y) \
                * count_visible_more_y(input_list, x, y) \
                * count_visible_less_y(input_list, x, y)            

            if scenic_score > max_scenic_score:
                max_scenic_score = scenic_score

    print(f'Part two - : {max_scenic_score}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r", encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
