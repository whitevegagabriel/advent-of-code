#!/bin/python3


def has_neighbor_block(point, points):
    for p in [
        (point[0] + 1, point[1], point[2]),
        (point[0] + 1, point[1] + 1, point[2]),
        (point[0] + 1, point[1] - 1, point[2]),
        (point[0] + 1, point[1], point[2] + 1),
        (point[0] + 1, point[1], point[2] - 1),
        (point[0] - 1, point[1], point[2]),
        (point[0] - 1, point[1] + 1, point[2]),
        (point[0] - 1, point[1] - 1, point[2]),
        (point[0] - 1, point[1], point[2] + 1),
        (point[0] - 1, point[1], point[2] - 1),
        (point[0], point[1] + 1, point[2]),
        (point[0], point[1] + 1, point[2] + 1),
        (point[0], point[1] + 1, point[2] - 1),
        (point[0], point[1] - 1, point[2]),
        (point[0], point[1] - 1, point[2] + 1),
        (point[0], point[1] - 1, point[2] - 1),
        (point[0], point[1], point[2] + 1),
        (point[0], point[1], point[2] - 1),
    ]:
        if p in points:
            return True


def part_one(input_list):
    points = set()

    for point in input_list:
        points.add(tuple([int(num) for num in point.split(",")]))

    sides = 6 * len(input_list)

    for point in points:
        for p in [
            (point[0] + 1, point[1], point[2]),
            (point[0], point[1] + 1, point[2]),
            (point[0], point[1], point[2] + 1),
        ]:
            if p in points:
                sides -= 2

    return sides


def part_two(input_list):
    points = set()

    max_x = -1000000
    outside_point = None

    for point in input_list:
        p = tuple([int(num) for num in point.split(",")])
        if p[0] > max_x:
            max_x = max(max_x, p[0])
            outside_point = (p[0] + 1, p[1], p[2])
        points.add(p)

    sides = 0

    visited = {outside_point}
    to_visit = [outside_point]

    while len(to_visit) > 0:
        point = to_visit.pop()
        for p in [
            (point[0] + 1, point[1], point[2]),
            (point[0], point[1] + 1, point[2]),
            (point[0], point[1], point[2] + 1),
            (point[0] - 1, point[1], point[2]),
            (point[0], point[1] - 1, point[2]),
            (point[0], point[1], point[2] - 1),
        ]:
            if p in points:
                sides += 1
            elif p not in visited and has_neighbor_block(p, points):
                to_visit.append(p)
                visited.add(p)

    return sides


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    print(f"Part one - : {part_one(contents_list)}")
    print(f"Part two - : {part_two(contents_list)}")
