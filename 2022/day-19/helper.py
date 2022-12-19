import operator
from copy import copy


def parse_ints_from_string(chars):
    ints = []
    buffer = ""
    hasdigit = False
    for c in chars:
        if c.isdigit():
            buffer += c
            hasdigit = True
        elif hasdigit:
            ints.append(int(buffer))
            buffer = ""
            hasdigit = False
    if hasdigit:
        ints.append(int(buffer))
    return ints


def tuplify(listy_input):
    if not isinstance(listy_input, list) and not isinstance(listy_input, tuple):
        return listy_input

    if isinstance(listy_input, list):
        listy_input = copy(listy_input)
    else:
        listy_input = list(listy_input)

    for i, item in enumerate(listy_input):
        listy_input[i] = tuplify(item)

    return tuple(listy_input)


def listify(tuply_input):
    if not isinstance(tuply_input, list) and not isinstance(tuply_input, tuple):
        return tuply_input

    if isinstance(tuply_input, list):
        listy_input = copy(tuply_input)
    else:
        listy_input = list(tuply_input)

    for i, item in enumerate(listy_input):
        listy_input[i] = listify(item)

    return listy_input


def subtract_tuples(tuple1, tuple2, quantity=1):
    for _ in range(quantity):
        tuple1 = tuple(map(operator.sub, tuple1, tuple2))
    return tuple1


def add_tuples(tuple1, tuple2, quantity=1):
    for _ in range(quantity):
        tuple1 = tuple(map(operator.add, tuple1, tuple2))
    return tuple1
