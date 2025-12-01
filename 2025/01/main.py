def part_one():
    position = 50
    zero_counter = 0

    for line in open("input-1.txt"):
        line = line.strip()

        direction = line[0]
        value = int(line[1:])

        if direction == "R":
            position += value
        elif direction == "L":
            position -= value

        position %= 100

        # print(f"Current position: {position}")

        if position == 0:
            zero_counter += 1

    print(f"Final position: {position}")
    print(f"Zero counter: {zero_counter}")


def part_two():
    position = 50
    zero_counter = 0

    for line in open("input-1-test.txt"):
        line = line.strip()

        direction = line[0]
        value = int(line[1:])

        position += value

    zero_counter = position // 100

    print(f"Final position: {position}")
    print(f"Zero counter: {zero_counter}")


if __name__ == "__main__":
    # part_one()
    part_two()
