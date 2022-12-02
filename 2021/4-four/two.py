# Day Four - Part Two

import numpy as np


if __name__ == "__main__":

    bingo_boards = []

    sum_of_unmarked = 0
    last_num = 0

    winning_board = None
    found = False

    with open("input-test.txt", "r") as input_file:

        numbers = [int(num) for num in input_file.readline().strip("\n").split(",")]
        _ = input_file.readline()

        count = 0
        board = np.zeros((5, 5), dtype=np.int8)
        _board = np.copy(board)

        for line in [x.strip("\n") for x in input_file.readlines()]:
            if line == "":
                bingo_boards.append(_board)
                _board = np.copy(board)
                count = 0
            else:
                _board[count, :] = [int(num) for num in line.split()]
                count += 1
        bingo_boards.append(_board)

    print("\nchecking numbers now\n")

    for num in numbers:
        if found:
            break

        print(f'\nchecking {num}')

        for b in bingo_boards:  # [:-(len(bingo_boards)-1)]:
            if found:
                break

            for index, value in np.ndenumerate(b):
                if value == num:
                    print(f'found {num} in bingo board')
                    b[index] = -1

            # Check if row or column is full of -1's.
            for i in range(5):
                if np.all(b[i, :] == -1) or np.all(b[:, i] == -1):
                    print("row or column is -1")
                    winning_board = b
                    last_num = num
                    found = True

    sum_of_unmarked = np.sum(winning_board[winning_board >= 0])

    print(f'Final Score = {sum_of_unmarked * last_num}')
