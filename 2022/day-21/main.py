#!/bin/python3
import operator
import sys
import helper


def get_operations(operation_list):
    operations = dict()
    for operation in operation_list:
        key = operation[0:4]
        if len(operation) == 17:
            value1 = operation[6:10]
            value2 = operation[13:17]
            op = operation[11]
            if op == "+":
                op = operator.add
            elif op == "-":
                op = operator.sub
            elif op == "*":
                op = operator.mul
            else:
                op = operator.truediv
            value = [value1, value2, op]
        else:
            value = int(operation.split()[1])

        operations[key] = value
    return operations


def get_result(key, operations):
    value = operations[key]
    if isinstance(value, int):
        return value
    num1 = get_result(value[0], operations)
    num2 = get_result(value[1], operations)
    return value[2](num1, num2)


def compare(humn, operations):
    operations["humn"] = humn

    value1 = operations["root"][0]
    value2 = operations["root"][1]

    res1 = get_result(value1, operations)
    res2 = get_result(value2, operations)

    return res1 - res2


def get_directionality(operations):
    humn = 0
    starting_res = compare(humn, operations)
    res = starting_res
    while res == starting_res:
        humn += 1
        res = compare(humn, operations)
    return abs(starting_res - res) // (starting_res - res)


"""
Performs a recursive search. For large inputs, could cache intermediate results.
"""


def part_one(input_list):
    operations = get_operations(input_list)
    return int(get_result("root", operations))


"""
Leverages the recursive search from part one, then performs a binary search over the solution space. A more efficient
solution would generate exact functions for the left / right hand sides of the equation, then solve.
"""


def part_two(input_list):
    operations = get_operations(input_list)

    lo = 0
    hi = 10000000000000
    mid = lo + (hi - lo) // 2
    directionality = get_directionality(operations)
    while (res := compare(mid, operations)) != 0:
        if res * directionality > 0:
            lo = mid + 1
            if lo == hi:
                hi = hi * 2
        elif res * directionality < 0:
            if lo == hi:
                lo = lo - hi
            hi = mid - 1

        mid = lo + (hi - lo) // 2
    return mid


if __name__ == "__main__":
    input_file = "input.txt"
    if len(sys.argv) == 2 and sys.argv[1] == "test":
        input_file = "test.txt"

    contents_list = open(input_file, "r", encoding="utf-8").read().splitlines()
    print(f"Part one - : {part_one(contents_list)}")
    print(f"Part two - : {part_two(contents_list)}")
