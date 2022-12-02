# Day Three - Part Two


if __name__ == "__main__":

    oxygen_rating: int = 0
    co2_rating: int = 0

    with open("input.txt", "r") as input_file:
        value_list = [x.strip("\n") for x in input_file.readlines()]
        oxygen_rating_list = value_list
        co2_rating_list = value_list

    bit_length: int = len(value_list[0])
    print(f'{bit_length = }')

    # Oxygen Rating:

    round_count: int = 0

    while len(value_list) > 1:

        one_count: list[int] = [0 for _ in range(bit_length)]
        zero_count: list[int] = [0 for _ in range(bit_length)]

        print(f'\n{round_count = }')
        print(f'(before) {value_list = }')

        for index, value in enumerate(value_list):

            for i in range(bit_length):
                if value[i] == "0":
                    zero_count[i] += 1
                else:
                    one_count[i] += 1

        if one_count[round_count] >= zero_count[round_count]:
            value_list = [x for x in value_list if x[round_count] == "1"]
        else:
            value_list = [x for x in value_list if x[round_count] == "0"]

        print(f'(after) {value_list = }')

        round_count += 1

    # CO2 Rating:

    round_count: int = 0

    while len(co2_rating_list) > 1:

        one_count: list[int] = [0 for _ in range(bit_length)]
        zero_count: list[int] = [0 for _ in range(bit_length)]

        print(f'\n{round_count = }')
        print(f'(before) {co2_rating_list = }')

        for index, value in enumerate(co2_rating_list):

            for i in range(bit_length):
                if value[i] == "0":
                    zero_count[i] += 1
                else:
                    one_count[i] += 1

        if one_count[round_count] >= zero_count[round_count]:
            co2_rating_list = [x for x in co2_rating_list if x[round_count] == "0"]
        else:
            co2_rating_list = [x for x in co2_rating_list if x[round_count] == "1"]

        print(f'(after) {co2_rating_list = }')

        round_count += 1

    oxygen_rating = int(value_list[0], 2)
    print(f'\n{oxygen_rating = }')

    co2_rating = int(co2_rating_list[0], 2)
    print(f'{co2_rating = }')

    life_support_rating: int = oxygen_rating * co2_rating
    print(f'\n{life_support_rating = }')
