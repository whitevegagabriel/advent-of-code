#!/bin/python3
import helper
from copy import deepcopy
from collections import deque


def get_blueprints(blueprint_list):
    # ( 0 ,   1 ,     2   ,   3  )
    # (ore, clay, obsidian, geode)
    blueprints = [tuple() for _ in range(len(blueprint_list))]
    for i, line in enumerate(blueprint_list):
        ints = helper.parse_ints_from_string(line)
        blueprints[i] = (
            (ints[1], 0, 0, 0),  # 0
            (ints[2], 0, 0, 0),  # 1
            (ints[3], ints[4], 0, 0),  # 2
            (ints[5], 0, ints[6], 0),  # 3
        )
    return blueprints


def get_max_costs(blueprint):
    max_costs = [0] * 4
    for costs in blueprint:
        for i, cost in enumerate(costs):
            max_costs[i] = max(max_costs[i], cost)
    return max_costs


def get_theoretical_delta(state):
    if state[2] == 0:
        return 0

    time_left = state[2]

    return (time_left - 1)**2 / 2


def get_next_state(state, resources, robot_type, fastforward=0):
    new = deepcopy(state)
    new[0] = list(resources)
    new[2] -= fastforward + 1
    new[0] = helper.add_tuples(new[0], new[1])
    if robot_type == 3:
        new[3] += new[2]
    else:
        new[1][robot_type] += 1
    return new


def append_if_valid(state, curr_max, queue):
    curr_geodes = state[3]
    min_possible_geodes = curr_geodes
    unrealistic_theoretical_maximum = min_possible_geodes + get_theoretical_delta(
        state)

    if unrealistic_theoretical_maximum >= curr_max:
        queue.appendleft(state)


def get_max_geodes(blueprint, state):
    # ( 0 ,   1 ,     2   ,   3  )
    # (ore, clay, obsidian, geode)

    to_see = deque()
    to_see.appendleft(helper.listify(state))
    max_ore_cost = max(robot_cost[0] for robot_cost in blueprint)

    max_geodes = 0
    while len(to_see) > 0:
        curr = to_see.pop()

        stock_tuple = tuple(curr[0])
        ore_stock, clay_stock, obsidian_stock = stock_tuple

        robot_tuple = tuple(curr[1])
        ore_robots, clay_robots, obsidian_robots = robot_tuple

        curr_mins_left = curr[2]
        curr_geodes = curr[3]
        max_geodes = max(max_geodes, curr_geodes)

        # state ran out of time
        if curr_mins_left < 2:
            continue

        for robot_type, robot_cost in enumerate(blueprint):
            # no obsidian bots, can never build geode yet
            if robot_type == 3 and obsidian_robots == 0:
                continue
            # if production of obsidian + stock exceeds cost to build one geode robot until rounds run out
            elif robot_type == 2 and ((
                    obsidian_robots * curr_mins_left +
                    obsidian_stock) >= blueprint[3][2] * curr_mins_left or clay_robots == 0):
                continue
            # if production of clay + stock exceeds cost to build one obsidian robot until rounds run out
            elif robot_type == 1 and (clay_robots * curr_mins_left + clay_stock
                                   ) >= blueprint[2][1] * curr_mins_left:
                continue
            # if production of ore + stock exceeds cost to build any robot once until rounds run out
            elif robot_type == 0 and (ore_robots * curr_mins_left +
                                   ore_stock) >= max_ore_cost * curr_mins_left:
                continue

            new_resources = helper.subtract_tuples(stock_tuple, robot_cost)
            fast_forward_amt = 0
            while not all([item >= 0 for item in new_resources]):
                new_resources = helper.add_tuples(new_resources, robot_tuple)
                fast_forward_amt += 1

            if fast_forward_amt > curr_mins_left:
                continue

            fast_forward = get_next_state(curr, new_resources, robot_type,
                                          fast_forward_amt)
            append_if_valid(fast_forward, max_geodes, to_see)

    return max_geodes


def prod(nums):
    product = 1
    for num in nums:
        product = product * num
    return product


def get_all_max_geodes(blueprints, minutes):
    max_geodes = []
    state = (
        (0, 0, 0),  # resources
        (1, 0, 0),  # robots
        minutes,  # minutes
        0,
    )

    for bp in blueprints:
        new_max_geodes = get_max_geodes(bp, state)
        max_geodes.append(new_max_geodes)
    return max_geodes


def part_one(input_list):
    blueprints = get_blueprints(input_list)

    max_geodes = get_all_max_geodes(blueprints, 24)

    prod_sum = 0
    for i, m in enumerate(max_geodes):
        prod_sum += (i + 1) * m

    return prod_sum


def part_two(input_list):
    blueprints = get_blueprints(input_list[:3])
    max_geodes = get_all_max_geodes(blueprints, 32)
    product = prod(max_geodes)
    return product


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    print(f"Part one - : {part_one(contents_list)}")
    print(f"Part two - : {part_two(contents_list)}")
