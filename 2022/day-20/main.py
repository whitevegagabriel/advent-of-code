#!/bin/python3
import helper


def mix(nums, positions):
    positions = positions[:]
    length = len(nums)
    for original_pos, num in enumerate(nums):
        if num == 0:
            continue

        old_pos = positions[original_pos]
        shift_amt = abs(num)  # - 1
        shift_sign = num // abs(num)

        new_pos = (old_pos + shift_amt * shift_sign) % (length - 1)
        lo = min(old_pos, new_pos)
        hi = max(old_pos, new_pos)
        delta = 1
        if old_pos <= new_pos:
            delta = -1
            hi += 1
        else:
            delta = 1
            lo -= 1
        for i, pos in enumerate(positions):
            if lo < pos < hi:
                positions[i] = (positions[i] + delta) % length
        positions[original_pos] = new_pos

    return positions


def get_grove_coords(nums, qty):
    length = len(nums)

    shifted_positions = list(range(length))
    for _ in range(qty):
        shifted_positions = mix(nums, shifted_positions)

    shifted_nums = [0] * length
    for i, position in enumerate(shifted_positions):
        shifted_nums[position] = nums[i]

    pos_of_zero = 0
    for pos, num in enumerate(shifted_nums):
        if num == 0:
            pos_of_zero = pos
            break

    length = len(shifted_nums)
    grove_coords = [
        shifted_nums[(pos_of_zero + 1000) % length],
        shifted_nums[(pos_of_zero + 2000) % length],
        shifted_nums[(pos_of_zero + 3000) % length],
    ]

    return grove_coords


def part_one(input_list):
    nums = [int(num, 10) for num in input_list]
    grove_coords = get_grove_coords(nums, 1)
    return sum(grove_coords)


def part_two(input_list):
    decrypt_key = 811589153
    nums = [int(num, 10) * decrypt_key for num in input_list]
    grove_coords = get_grove_coords(nums, 10)
    return sum(grove_coords)


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    print(f"Part one - : {part_one(contents_list)}")
    print(f"Part two - : {part_two(contents_list)}")
