#!/bin/python3


class Dir:
    name = ""
    contents = None
    parent = None

    def __init__(self, name):
        self.name = name
        self.contents = []

    def get_size(self):
        return sum([item.get_size() for item in self.contents])

    def get_child_dir(self, name):
        for item in self.contents:
            if item.name == name and isinstance(item, Dir):
                return item

    def add_child(self, new_item):

        new_item.parent = self
        self.contents.append(new_item)


class File:
    name = ""
    size = 0
    parent = None

    def __init__(self, size, name):
        self.name = name
        self.size = size

    def get_size(self):
        return self.size

    def equals(self, other):
        return (self.name == other.name and self.parent == other.parent
                and isinstance(other, File))


def create_item_from_command(command):
    if command[0] == "dir":
        return Dir(command[1])
    else:
        return File(int(command[0]), command[1])


def get_filetree(commands):
    root = Dir("/")
    curr_dir = root

    for rawcommand in commands[1:]:
        command = rawcommand.split()
        if not command[0] == "$":
            curr_dir.add_child(create_item_from_command(command))
        elif command[1] == "cd":
            new_dir_name = command[2]
            if new_dir_name == "..":
                curr_dir = curr_dir.parent
            else:
                curr_dir = curr_dir.get_child_dir(new_dir_name)

    return root


def get_directories(top_level_directory):
    directories = [top_level_directory]

    for item in top_level_directory.contents:
        if isinstance(item, Dir):
            directories += get_directories(item)

    return directories


def part_one(input_list):
    filetree = get_filetree(input_list)
    directories = get_directories(filetree)

    dir_sum = sum(
        [dir.get_size() for dir in directories if dir.get_size() <= 100000])

    print(f'Part one - : {dir_sum}')


def part_two(input_list):
    filetree = get_filetree(input_list)
    directories = get_directories(filetree)

    required_space = filetree.get_size() + 30_000_000 - 70_000_000
    min_deletable_folder_size = min([
        item.get_size() for item in directories
        if item.get_size() >= required_space
    ])

    print(f'Part two - : {min_deletable_folder_size}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
