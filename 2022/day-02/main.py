#!/bin/python3


class Choice():
    rock = 0
    paper = 1
    scissors = 2
    identity = 0
    value = 0

    def __init__(self, identity_str: str):
        self.identity = self.parse(identity_str)
        self.value = self.identity + 1

    def parse(self, choice_str: str) -> int:
        if choice_str in ["X", "A"]:
            return self.rock
        elif choice_str in ["Y", "B"]:
            return self.paper
        elif choice_str in ["Z", "C"]:
            return self.scissors

    def beats(self, other) -> bool:
        return (self.identity - 1) % 3 == other.identity

    def equals(self, other) -> bool:
        return self.identity == other.identity

    def inc(self):
        self.identity = (self.identity + 1) % 3
        self.value = self.identity + 1

    def dec(self):
        self.identity = (self.identity - 1) % 3
        self.value = self.identity + 1


def part_one(input_list: list):
    total_score = 0
    for choices in input_list:
        your_choice = Choice(choices[0])
        my_choice = Choice(choices[2])

        score_add = my_choice.value

        if my_choice.equals(your_choice):
            score_add += 3
        elif my_choice.beats(your_choice):
            score_add += 6

        total_score += score_add

    print(f"Part one - : {total_score}")


def part_two(input_list: list):
    total_score = 0
    for choices in input_list:
        your_choice = Choice(choices[0])
        my_choice = Choice(choices[0])

        if choices[2] == "X":
            my_choice.dec()
        elif choices[2] == "Z":
            my_choice.inc()

        score_add = my_choice.value

        if my_choice.equals(your_choice):
            score_add += 3
        elif my_choice.beats(your_choice):
            score_add += 6

        total_score += score_add

    print(f"Part two - : {total_score}")


if __name__ == "__main__":
    contents_list = open("input.txt").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
