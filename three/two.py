# Day Three - Part Two


if __name__ == "__main__":

    life_support_rating: int = 0
    oxygen_rating: int = 0
    co2_rating: int = 0

    zero_count: list[int] = []
    one_count: list[int] = []

    value_list: list[str] = []

    length: int = 0

    with open("input-test.txt", "r") as input_file:

        length = len(input_file.readline().strip("\n"))

        zero_count = [0 for x in range(length)]
        one_count = [0 for x in range(length)]

        for line in (lines := input_file.readlines()):

            line = line.strip("\n")

            for i in range(length):
                if line[i] == "0":
                    zero_count[i] += 1
                else:
                    one_count[i] += 1

            # print(f'{line = }')

    value_list = [x.strip("\n") for x in lines]

    print(f'{zero_count = }, {one_count = }')

    print(f'{value_list = }')

    for i in range(length):
        if len(value_list) == 1:
            break

        zero_count = [0 for x in range(length)]
        one_count = [0 for x in range(length)]

        for j in range(length):
            if line[j] == "0":
                zero_count[j] += 1
            else:
                one_count[j] += 1

        print(f'{zero_count = }, {one_count = }')

        if one_count[i] >= zero_count[i]:
            value_list = [x for x in value_list if x[i] == "1"]
        else:
            value_list = [x for x in value_list if x[i] == "0"]

        print(f'({i}) {value_list = }')

    life_support_rating = oxygen_rating * co2_rating
    print(f'{life_support_rating = }')
