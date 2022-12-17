#!/bin/python3


def find_position(grid, character):
    for x, elevation_list in enumerate(grid):
        for y, elevation in enumerate(elevation_list):
            if elevation == character:
                return (x, y)


def is_at_most_one_step_higher(grid, source, dest):
    source_elev = grid[source[0]][source[1]]
    dest_elev = grid[dest[0]][dest[1]]

    if source_elev == "S":
        source_elev = "a"
    elif source_elev == "E":
        source_elev = "z"

    if dest_elev == "S":
        dest_elev = "a"
    elif dest_elev == "E":
        dest_elev = "z"

    return ord(dest_elev) - ord(source_elev) <= 1


def is_at_most_one_step_lower(grid, source, dest):
    source_elev = grid[source[0]][source[1]]
    dest_elev = grid[dest[0]][dest[1]]

    if source_elev == "S":
        source_elev = "a"
    elif source_elev == "E":
        source_elev = "z"

    if dest_elev == "S":
        dest_elev = "a"
    elif dest_elev == "E":
        dest_elev = "z"

    return ord(dest_elev) - ord(source_elev) >= -1


def get_possible_moves_part_1(grid, location):
    moves = []

    up = (location[0] - 1, location[1])
    down = (location[0] + 1, location[1])
    left = (location[0], location[1] - 1)
    right = (location[0], location[1] + 1)

    if (up[0] >= 0 and is_at_most_one_step_higher(grid, location, up)):
        moves.append(up)
    if (down[0] < len(grid)
            and is_at_most_one_step_higher(grid, location, down)):
        moves.append(down)
    if (left[1] >= 0 and is_at_most_one_step_higher(grid, location, left)):
        moves.append(left)
    if (right[1] < len(grid[0])
            and is_at_most_one_step_higher(grid, location, right)):
        moves.append(right)
    return moves


def get_possible_moves_part_2(grid, location):
    moves = []

    up = (location[0] - 1, location[1])
    down = (location[0] + 1, location[1])
    left = (location[0], location[1] - 1)
    right = (location[0], location[1] + 1)

    if (up[0] >= 0 and is_at_most_one_step_lower(grid, location, up)):
        moves.append(up)
    if (down[0] < len(grid)
            and is_at_most_one_step_lower(grid, location, down)):
        moves.append(down)
    if (left[1] >= 0 and is_at_most_one_step_lower(grid, location, left)):
        moves.append(left)
    if (right[1] < len(grid[0])
            and is_at_most_one_step_lower(grid, location, right)):
        moves.append(right)
    return moves


def part_one(input_list):
    start = find_position(input_list, "S")
    destination = find_position(input_list, "E")

    visual_list = [list(elevations) for elevations in input_list]

    found_destination = False
    moves_made = -1
    places_visited = set()
    places_visited.add(start)
    to_visit = [start]
    while not found_destination and len(to_visit) > 0:
        moves_made += 1
        currently_visiting = to_visit[:]
        to_visit = []
        for location in currently_visiting:
            visual_list[location[0]][location[1]] = "*"
            if location == destination:
                found_destination = True
            possible_moves = get_possible_moves_part_1(input_list, location)
            for possible_move in possible_moves:
                if possible_move not in places_visited:
                    places_visited.add(possible_move)
                    to_visit.append(possible_move)

    if not found_destination:
        print("\n".join(["".join(elevations) for elevations in visual_list]))

    print(f'Part one - : {moves_made}')


def part_two(input_list):
    start = find_position(input_list, "E")

    visual_list = [list(elevations) for elevations in input_list]

    found_destination = False
    moves_made = -1
    places_visited = set()
    places_visited.add(start)
    to_visit = [start]
    while not found_destination and len(to_visit) > 0:
        moves_made += 1
        currently_visiting = to_visit[:]
        to_visit = []
        for location in currently_visiting:
            visual_list[location[0]][location[1]] = "*"
            elevation = input_list[location[0]][location[1]]
            if elevation == "a" or elevation == "S":
                found_destination = True
            possible_moves = get_possible_moves_part_2(input_list, location)
            for possible_move in possible_moves:
                if possible_move not in places_visited:
                    places_visited.add(possible_move)
                    to_visit.append(possible_move)

    if not found_destination:
        print("\n".join(["".join(elevations) for elevations in visual_list]))

    print(f'Part two - : {moves_made}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
