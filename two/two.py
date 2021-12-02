# Day Two - Part Two


if __name__ == "__main__":

    aim: int = 0

    horizontal: int = 0
    depth: int = 0

    with open("input.txt", "r") as input_file:
        for line in input_file.readlines():
            # print(f'{line = }')

            if "\n" in line:
                line = line[0:-1]
                # print(f'(edited) {line = }')

            value = int(line.split(" ")[1])

            if "forward" in line:
                horizontal += value
                depth += aim * value

            if "up" in line:
                aim -= value

            if "down" in line:
                aim += value

    print(f'distance = {horizontal * depth}')
