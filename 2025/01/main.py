def part_one():
    position = 50
    zero_counter = 0

    for line in open("input.txt"):
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


test_lines =  [
    # "R1000", # 10
    # "L50", "R200", # 3
    "L50", "R50", # 1
]

def part_two():
    position = 50
    zero_counter = 0

    for line in open("input.txt"):
    # for line in open("input-test.txt"):
    # for line in test_lines:
        line = line.strip()

        direction = line[0]
        value = int(line[1:])

        if direction == "R":
            for _ in range(value):
                position += 1
                position %= 100
                if position == 0:
                    zero_counter += 1
        elif direction == "L":
            for _ in range(value):
                position -= 1
                position %= 100
                if position == 0:
                    zero_counter += 1

        # position %= 100
        # print(f"Current position: {position}")

    print(f"Final position: {position}")
    print(f"Zero counter: {zero_counter}")


if __name__ == "__main__":
    # part_one()
    part_two()
