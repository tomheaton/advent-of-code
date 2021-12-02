# Day Two - Part One


if __name__ == "__main__":

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

            if "up" in line:
                depth -= value

            if "down" in line:
                depth += value

    print(f'distance = {horizontal * depth}')
