#!/bin/python3


def find_sensors(input_list):
    sensors = dict()
    for line in input_list:
        line = line.split(":")
        
        first_half = line[0].split(",")
        sensor_x = int(first_half[0].split("=")[1])
        sensor_y = int(first_half[1].split("=")[1])
        
        second_half = line[1].split(",")
        beacon_x = int(second_half[0].split("=")[1])
        beacon_y = int(second_half[1].split("=")[1])
        
        sensors[(sensor_x, sensor_y)] = abs(beacon_x - sensor_x) + abs(beacon_y - sensor_y)
    return sensors


def find_range_target_coords(sensor, target_y, reach):
    distance_to_target = abs(sensor[1] - target_y)

    horizontal_deviation = reach - distance_to_target
    if horizontal_deviation < 0:
        return None, None

    return sensor[0] - horizontal_deviation, sensor[0] + horizontal_deviation


def merge_ranges(new_range, min_max_range):
    min_val, max_val = new_range
    if len(min_max_range) == 0:
        min_max_range.append((min_val, max_val))
        return

    for i, i_range in enumerate(min_max_range):
        if min_val <= i_range[1] + 1 and max_val >= i_range[0] - 1:
            min_max_range.remove(i_range)
            new_range = (min(min_val, i_range[0]), max(max_val, i_range[1]))
            merge_ranges(new_range, min_max_range)
            return
        elif max_val < i_range[0] - 1:
            min_max_range.insert(i, (min_val, max_val))
            return

    min_max_range.append((min_val, max_val))


def calculate_ranges(sensors, target_y):
    min_max_range = []
    for sensor, distance in sensors.items():
        new_range = find_range_target_coords(sensor, target_y, distance)
        if new_range == (None, None):
            continue
        merge_ranges(new_range, min_max_range)
    return min_max_range


def part_one(input_list):
    sensors = find_sensors(input_list)

    target_y = 2_000_000

    ranges = calculate_ranges(sensors, target_y)

    return sum([item[1] - item[0] for item in ranges])


def part_two(input_list):
    sensors = find_sensors(input_list)

    max_val = 4_000_000

    for target_y in range(max_val, -1, -1):
        ranges = calculate_ranges(sensors, target_y)
        if len(ranges) == 2:
            return 4000000 * (ranges[0][1]+1) + target_y

    return None


if __name__ == "__main__":
    contents_list = open("input.txt", "r", encoding="utf-8").read().splitlines()
    print(f"Part one - : {part_one(contents_list)}")
    print(f"Part two - : {part_two(contents_list)}")
