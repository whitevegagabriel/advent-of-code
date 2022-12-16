#!/bin/python3
from collections import deque
from copy import copy
import time
from functools import lru_cache

def find_starting_valve(valve_list, starting_valve_name):
    valves = dict()
    next_valve_names = dict()
    for valve_desc in valve_list:
        valve_args = valve_desc \
            .replace("Valve ", "") \
            .replace("has flow rate=", "") \
            .replace("; tunnels lead to valves", "") \
            .replace("; tunnel leads to valve", "") \
            .replace(",", "") \
            .split()

        valves[valve_args[0]] = Valve(valve_args[0], int(valve_args[1]))
        next_valve_names[valve_args[0]] = valve_args[2:]

    for valve_name in valves.keys():
        for next_valve_name in next_valve_names[valve_name]:
            valves[valve_name].next_valves.add(valves[next_valve_name])
            
            
    unopened_valves = []
    for valve in valves.values():
        if valve.flow_rate > 0:
            unopened_valves.append(valve)
            
    unopened_valves.sort()
    return valves[starting_valve_name], tuple(unopened_valves)


class Valve:
    name = ""
    flow_rate = 0
    next_valves = None

    def __init__(self, name, flow_rate):
        self.flow_rate = flow_rate
        self.name = name
        self.next_valves = set()
        
    def __lt__(self, other):
        return self.name < other.name
    
    def __gt__(self, other):
        return self.name > other.name
    
    def __eq__(self, other):
        return self.name == other.name
    
    def __hash__(self):
        return hash(self.name)

@lru_cache(maxsize=None)
def distance_between(start, end):
    distance = -1
    visited = set()
    to_visit = deque()
    to_visit.appendleft(start)
    found = False
    while not found:
        num_to_visit = len(to_visit)
        distance += 1
        for _ in range(num_to_visit):
            curr = to_visit.pop()
            if curr == end:
                found = True
            visited.add(curr)
            for next_v in curr.next_valves:
                if next_v not in visited:
                    to_visit.appendleft(next_v)
                    
    return distance

def calculate_max_pressure_released_1(starting_valve, unopened_valves, starting_minutes, cache):
    names = [v.name for v in unopened_valves]
    names.sort()
    id = (starting_valve.name, ",".join(names), starting_minutes)
    
    if id in cache:
        return cache[id]
    
    max_pressure_released = 0
    
    for unopened_valve in unopened_valves:
        if unopened_valve == starting_valve:
            continue
        if starting_valve < unopened_valve:
            dist = distance_between(starting_valve, unopened_valve)
        else:
            dist = distance_between(unopened_valve, starting_valve)
        minutes_left = starting_minutes - dist - 1
        if minutes_left >= 0:
            new_unopened_valves = unopened_valves[:]
            new_unopened_valves.remove(unopened_valve)
            pressure_released = minutes_left * unopened_valve.flow_rate
            if minutes_left >= 2:
                pressure_released = pressure_released + calculate_max_pressure_released_1(unopened_valve, new_unopened_valves, minutes_left, cache)
            max_pressure_released = max(max_pressure_released, pressure_released)
        
    
    cache[id] = max_pressure_released    
    
    return max_pressure_released


def complement(valves, superset):
    complement = list(superset)
    for valve in valves:
        complement.remove(valve)
    return complement


def stringify(valves):
    key = [str(valve) for valve in valves]
    key.sort()
    return ",".join(key)


#@lru_cache(maxsize=1000000)
def calculate_max_pressure_released_2(starting_valve, unopened_valves, starting_minutes, max_scores, superset):
    for unopened_valve in unopened_valves:
        new_unopened_valves = list(unopened_valves)
        new_unopened_valves.remove(unopened_valve)
        key = stringify(new_unopened_valves)
        key_complement = stringify(new_unopened_valves)
        if key not in max_scores and key_complement not in max_scores:
            max_scores[key] = calculate_max_pressure_released_1(starting_valve, new_unopened_valves, starting_minutes, dict())
            max_scores[key] += calculate_max_pressure_released_1(starting_valve, complement(new_unopened_valves, superset), starting_minutes, dict())
            max_scores[key_complement] = max_scores[key]
        
            calculate_max_pressure_released_2(starting_valve, tuple(new_unopened_valves), starting_minutes, max_scores, superset)


def part_one(input_list):
    starting_valve, unopened_valves = find_starting_valve(input_list, "AA")

    starting_minutes = 30

    return calculate_max_pressure_released_1(starting_valve, list(unopened_valves), starting_minutes, dict())


def part_two(input_list):
    starting_valve, unopened_valves = find_starting_valve(input_list, "AA")
    
    starting_minutes = 26
    
    max_scores = dict()
    
    calculate_max_pressure_released_2(starting_valve, unopened_valves, starting_minutes, max_scores, unopened_valves)

    return max(max_scores.values())

if __name__ == "__main__":
    contents_list = open("input.txt", "r").read().splitlines()
    print("Part one: " + str(part_one(contents_list)))
    print("Part two: " + str(part_two(contents_list)))
