#!/bin/python3


def offset_coords(x_offset, y_offset, coords):
    new_coords = set()
    for coord in coords:
        new_coord = (coord[0] + x_offset, coord[1] + y_offset)
        new_coords.add(new_coord)
    return new_coords


def move_piece(piece, move):
    x_mod = 0
    y_mod = 0
    if move == "<":
        x_mod = -1
    elif move == ">":
        x_mod = 1
    else:
        y_mod = -1
    return offset_coords(x_mod, y_mod, piece)


class Game:
    moves = ""
    curr_move = 0
    time_for_horz_move = False
    horz = {(0, 0), (1, 0), (2, 0), (3, 0)}
    plus = {(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)}
    ell = {(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)}
    vert = {(0, 0), (0, 1), (0, 2), (0, 3)}
    sqre = {(0, 0), (0, 1), (1, 0), (1, 1)}
    pieces = [horz, plus, ell, vert, sqre]
    curr_piece = 0
    board = None
    max_y_coord = -1

    def __init__(self, moves):
        self.moves = moves
        self.board = set()

    def simulate_piece(self):
        piece = self.get_piece()
        while True:
            move = self.get_move()
            moved_piece = move_piece(piece, move)
            overlap = self.intersects_board(moved_piece)

            if not overlap:
                piece = moved_piece
                continue
            elif move not in ["<", ">"]:
                break

        self.add_coords_to_board(piece)

    def intersects_board(self, piece):
        for coord in piece:
            if coord[0] < 0:
                return True
            if coord[0] > 6:
                return True
            if coord[1] < 0:
                return True
            if coord in self.board:
                return True
        return False

    def add_coords_to_board(self, coords):
        max_piece_coord = 0
        for coord in coords:
            max_piece_coord = max(max_piece_coord, coord[1])
            self.board.add(coord)
        self.max_y_coord = max(self.max_y_coord, max_piece_coord)

    def get_piece(self):
        piece = offset_coords(2, self.max_y_coord + 4,
                              self.pieces[self.curr_piece])
        self.curr_piece = (self.curr_piece + 1) % len(self.pieces)
        return piece

    def get_move(self):
        self.time_for_horz_move = not self.time_for_horz_move
        if self.time_for_horz_move:
            move = self.moves[self.curr_move]
            self.curr_move = (self.curr_move + 1) % len(self.moves)
            return move
        return "d"

    def get_state(self):
        state = [self.max_y_coord + 1] * 7
        for i in range(7):
            for j in range(self.max_y_coord + 1):
                if (i, self.max_y_coord - j) in self.board:
                    state[i] = j
                    break
        return tuple(state)

    def __str__(self):
        board_str = [["."] * 7 for _ in range(20)]
        board_str.append(["-"] * 7)
        for coord in self.board:
            board_str[coord[1]][coord[0]] = "#"
        board_str.reverse()
        return "\n".join(["".join(line) for line in board_str])


def find_max_height_for_iterations(moves, iterations):
    game = Game(moves)

    for i in range(iterations):
        game.simulate_piece()

    return game.max_y_coord + 1


def part_one(input_list):
    return find_max_height_for_iterations(input_list[0], 2022)


def part_two(input_list):
    game = Game(input_list[0])

    seen_game_states = dict()
    i = 0
    while True:
        game.simulate_piece()
        new_seen = (game.curr_move, game.curr_piece, game.get_state())
        if new_seen in seen_game_states:
            starting_height = seen_game_states[new_seen][0]
            current_height = game.max_y_coord + 1
            base_unit = current_height - starting_height
            starting_iteration = seen_game_states[new_seen][1]
            current_iteration = i
            break
        seen_game_states[new_seen] = (game.max_y_coord + 1, i)
        i += 1

    iters = 1000000000000
    base_iterations = current_iteration - starting_iteration
    num_base_iterations = (iters - starting_iteration) // base_iterations
    rem_base_iterations = (iters - starting_iteration) % base_iterations

    base_solution = (num_base_iterations - 1) * base_unit

    compelementary_solution = find_max_height_for_iterations(
        input_list[0], current_iteration + rem_base_iterations)

    return base_solution + compelementary_solution


if __name__ == "__main__":
    contents_list = open("input.txt", "r",
                         encoding="utf-8").read().splitlines()
    print(f"Part one - : {part_one(contents_list)}")
    print(f"Part two - : {part_two(contents_list)}")
