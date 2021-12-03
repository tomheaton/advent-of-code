# Day One - Part Two


if __name__ == "__main__":
    count: int = 0
    previous: int = 0
    current: int = 0

    three_list: [int] = []

    with open("input.txt", "r") as input_file:
        for index, line in enumerate(lines := [int(x) for x in input_file.readlines()]):
            try:
                three_list.append(lines[index] + lines[index + 1] + lines[index + 2])
            except IndexError as e:
                pass

    for number in three_list:
        current = number
        if current > previous != 0:
            count += 1
        previous = current

    print(f'{three_list = }')
    print(f'{count = }')
