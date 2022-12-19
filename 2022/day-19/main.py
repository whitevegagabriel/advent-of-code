#!/bin/python3
import helper
from copy import deepcopy
from collections import deque
from functools import lru_cache


def get_blueprints(blueprint_list):
    # ( 0 ,   1 ,     2   ,   3  )
    # (ore, clay, obsidian, geode)
    blueprints = [0 for _ in range(len(blueprint_list))]
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


def get_theoretical_delta(blueprint, state):
    if state[2] == 0:
        return 0

    time_left = state[2]

    return (time_left - 1)**2 / 2


@lru_cache(maxsize=100000)
def get_max_geodes(blueprint, state, max_robots=0, error_range=0, purchase_range=0):
    # ( 0 ,   1 ,     2   ,   3  )
    # (ore, clay, obsidian, geode)

    to_see = deque()
    to_see.appendleft(helper.listify(state))
    seen = {state}
    total_minutes = state[2]

    max_geodes = 0
    while len(to_see) > 0:
        curr = to_see.pop()

        curr_mins_left = curr[2]
        curr_geodes = curr[0][3]
        # geodes at end at current rate
        min_possible_geodes = curr_geodes + curr_mins_left * curr[1][3]
        max_geodes = max(max_geodes, min_possible_geodes)

        # figure out if curr can not possibly beat the maximum
        unrealistic_theoretical_maximum = min_possible_geodes + get_theoretical_delta(blueprint, curr)

        # state ran out of time
        if curr[2] == 0 or unrealistic_theoretical_maximum < max_geodes:
            continue

        could_afford_all_robots = True
        # purchase robots using original resources
        for robot_type, robot_cost in enumerate(blueprint):
            # if production of obsidian exceeds cost to build geode robot
            if robot_type < 3 and curr[1][2] >= blueprint[3][2]:
                continue
            # if production of clay exceeds cost to build obsidian robot
            if robot_type < 2 and curr[1][1] >= blueprint[2][1]:
                continue
            # if production of ore exceeds cost to build any robot
            if robot_type < 1 and curr[1][0] >= 4:
                continue
            new_resources = helper.subtract_tuples(tuple(curr[0]), robot_cost)
            # can afford robot
            if all([item >= 0 for item in new_resources]):
                new = deepcopy(curr)
                new[0] = list(new_resources)
                new[1][robot_type] += 1
                new[2] -= 1
                new[0] = helper.add_tuples(new[0], curr[1])

                curr_geodes = new[0][3]
                min_possible_geodes = curr_geodes + curr_mins_left * new[1][3]
                unrealistic_theoretical_maximum = min_possible_geodes + get_theoretical_delta(blueprint, new)

                if unrealistic_theoretical_maximum >= max_geodes:
                    tuplified = helper.tuplify(new)
                    if tuplified not in seen:
                        to_see.appendleft(new)
                        seen.add(tuplified)
            else:
                could_afford_all_robots = False

        if not could_afford_all_robots:
            new = deepcopy(curr)
            new[2] -= 1
            new[0] = helper.add_tuples(new[0], new[1])
            tuplified = helper.tuplify(new)
            if tuplified not in seen:
                to_see.appendleft(new)
                seen.add(tuplified)

    return max_geodes


def prod(nums):
    product = 1
    for num in nums:
        product = product * num
    return product


def get_all_max_geodes(blueprints, minutes):
    max_geodes = []
    state = (
        (0, 0, 0, 0),  # resources
        (1, 0, 0, 0),  # robots
        minutes  # minutes
    )

    for i, bp in enumerate(blueprints):
        new_max_geodes = get_max_geodes(bp, state)
        max_geodes.append(new_max_geodes)
        print(f"bp-{i}: {new_max_geodes}")
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
    #print(f"Part one - : {part_one(contents_list)}")
    print(f"Part two - : {part_two(contents_list)}")
