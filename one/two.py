# Day One - Part Two


if __name__ == "__main__":
    count: int = 0
    previous: int = 0
    current: int = 0

    with open("input.txt", "r") as input_file:
        for line in input_file.readlines():
            x = int(line)
            current = x
            if current > previous != 0:
                count += 1
            previous = current

    print(f'{count = }')
