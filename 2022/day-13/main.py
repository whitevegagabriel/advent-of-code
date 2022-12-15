#!/bin/python3


def listify(index, string):
    # Assume the first element is "["
    index += 1
    output = []
    buffered_num = ""
    # Iterate until we find the end of the list.
    while string[index] != "]":
        current_char = string[index]
        if current_char == "[":
            # Listify the sub-list, getting the index of the next position after it
            sublist, index = listify(index, string)
            output.append(sublist)
        elif current_char == ",":
            # Convert the buffered number to an integer and append to our list
            if buffered_num != "":
                output.append(int(buffered_num))
                buffered_num = ""
            index += 1
        else:
            # This is a digit that needs to be buffered
            buffered_num += current_char
            index += 1

    if buffered_num != "":
        output.append(int(buffered_num))

    return output, index + 1


def is_in_right_order(sub_packet_one, sub_packet_two):
    # Perform integer comparison if possible.
    if isinstance(sub_packet_one, int) and isinstance(
            sub_packet_two, int):
        if sub_packet_one == sub_packet_two:
            # inconclusive
            return None
        elif sub_packet_one < sub_packet_two:
            return True
        return False

    # Convert int to list
    if isinstance(sub_packet_one, int):
        sub_packet_one = [sub_packet_one]
    elif isinstance(sub_packet_two, int):
        sub_packet_two = [sub_packet_two]

    for index in range(min(len(sub_packet_one), len(sub_packet_two))):
        sub_packet_one_item = sub_packet_one[index]
        sub_packet_two_item = sub_packet_two[index]

        right_order = is_in_right_order(sub_packet_one_item, sub_packet_two_item)
        if right_order is not None:
            return right_order

    if len(sub_packet_one) == len(sub_packet_two):
        # inconclusive
        return None
    elif len(sub_packet_one) < len(sub_packet_two):
        return True

    return False


def partition(unsorted, low, high):
    pivot = unsorted[high]
    swap_index = low
    for index, item in enumerate(unsorted[low:high]):
        if is_in_right_order(item, pivot):
            unsorted[low+index] = unsorted[swap_index]
            unsorted[swap_index] = item
            swap_index += 1

    unsorted[high] = unsorted[swap_index]
    unsorted[swap_index] = pivot
    return swap_index


def quick_sort(unsorted, low, high):
    if low < high:
        partition_index = partition(unsorted, low, high)
        quick_sort(unsorted, low, partition_index - 1)
        quick_sort(unsorted, partition_index + 1, high)


def part_one(input_list):
    index_sum = 0

    for i in range(0, len(input_list) + 1, 3):
        packet_one = listify(0, input_list[i])[0]
        packet_two = listify(0, input_list[i + 1])[0]
        if is_in_right_order(packet_one, packet_two):
            index_sum += i // 3 + 1

    print(f'Part one - : {index_sum}')


def part_two(input_list):
    packets = [[[2]], [[6]]]
    for i in range(0, len(input_list) + 1, 3):
        packets.append(listify(0, input_list[i])[0])
        packets.append(listify(0, input_list[i+1])[0])

    quick_sort(packets, 0, len(packets)-1)

    decoder_key = 1
    for index, packet in enumerate(packets):
        if packet == [[2]] or packet == [[6]]:
            decoder_key = decoder_key * (index + 1)

    print(f'Part two - : {decoder_key}')


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    part_one(contents_list)
    part_two(contents_list)
