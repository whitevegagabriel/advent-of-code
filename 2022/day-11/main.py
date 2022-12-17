#!/bin/python3


def get_operation_lambda(operation):
    if "*" in operation:
        operation = operation.split("* ")
        if operation[1] == "old":
            return lambda num: num * num
        return lambda num: num * int(operation[1])
    else:
        operation = operation.split("+ ")
        return lambda num: num + int(operation[1])


def get_prod_of_max_two(numbers):
    max_nums = [0, 0]
    for num in numbers:
        if num > max_nums[0]:
            max_nums[1] = max_nums[0]
            max_nums[0] = num
        elif num > max_nums[1]:
            max_nums[1] = num

    return max_nums[0] * max_nums[1]


def create_monkeys(monkey_descriptions):
    monkey_list = MonkeyList()

    parse_length = 7
    for i in range(0, len(monkey_descriptions), parse_length):
        description = monkey_descriptions[i:i + parse_length]

        monkey_list.append(Monkey())
        monkey_list.monkeys[-1].identity = int(description[0].split()[1][:-1])
        monkey_list.monkeys[-1].items = [
            int(num) for num in description[1].split(":")[1].split(",")
        ]
        monkey_list.monkeys[-1].operation = get_operation_lambda(
            description[2].split("=")[1])
        monkey_list.monkeys[-1].post_operation = lambda num: num // 3
        monkey_list.monkeys[-1].divisibility = int(
            description[3].split("by ")[1])
        monkey_list.monkeys[-1].test_true_target = int(
            description[4].split("monkey ")[1])
        monkey_list.monkeys[-1].test_false_target = int(
            description[5].split("monkey ")[1])

    modulo = 1
    for monkey in monkey_list.monkeys:
        modulo = modulo * monkey.divisibility

    for monkey in monkey_list.monkeys:
        monkey.modulo = modulo

    return monkey_list


class MonkeyList:
    monkeys = None

    def __init__(self):
        self.monkeys = []

    def pass_item_to(self, item, monkey_id):
        self.monkeys[monkey_id].add_item(item)

    def append(self, monkey):
        self.monkeys.append(monkey)


class Monkey:
    identity = None
    items = None
    operation = None
    post_operation = None
    divisibility = 1
    test = None
    test_true_target = None
    test_false_target = None
    inspected_count = 0
    modulo = 1

    def __init__(self):
        self.test = lambda num: num % self.divisibility == 0

    def inspect_items(self, monkey_list):
        for item in self.items:
            self.inspected_count += 1
            worry_level = self.operation(item)
            worry_level = self.post_operation(worry_level)
            if self.test(worry_level):
                worry_level = worry_level % self.modulo
                monkey_list.pass_item_to(worry_level, self.test_true_target)
            else:
                monkey_list.pass_item_to(worry_level, self.test_false_target)

        self.items = []

    def add_item(self, item):
        self.items.append(item)


def part_one(input_list):
    monkey_list = create_monkeys(input_list)

    for i in range(20):
        for monkey in monkey_list.monkeys:
            monkey.inspect_items(monkey_list)

    counts = [monkey.inspected_count for monkey in monkey_list.monkeys]
    product = get_prod_of_max_two(counts)

    print(f'Part one - : {product}')


def part_two(input_list):
    monkey_list = create_monkeys(input_list)
    for monkey in monkey_list.monkeys:
        monkey.post_operation = lambda num: num

    for i in range(10000):
        for monkey in monkey_list.monkeys:
            monkey.inspect_items(monkey_list)

    counts = [monkey.inspected_count for monkey in monkey_list.monkeys]
    product = get_prod_of_max_two(counts)

    print(f'Part two - : {product}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
