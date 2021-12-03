# Day Three - Part One


if __name__ == "__main__":

    power_consumption: int = 0

    gamma_raw: str = ""
    epsilon_raw: str = ""

    gamma_rate: int = 0
    epsilon_rate: int = 0

    zero_count: list[int] = []
    one_count: list[int] = []

    with open("input.txt", "r") as input_file:

        length = input_file.readline().strip("\n")

        zero_count = [0 for x in length]
        one_count = [0 for x in length]

        for line in input_file.readlines():

            if "\n" in line:
                line = line[0:-1]

            for i in range(len(line)):
                if line[i] == "0":
                    zero_count[i] += 1
                else:
                    one_count[i] += 1

            print(f'{line = }')

    for index, number in enumerate(zero_count):
        # if number == one_count[index]:
        #     print("same")

        if number > one_count[index]:
            gamma_raw += "0"
            epsilon_raw += "1"
        else:
            epsilon_raw += "0"
            gamma_raw += "1"

    print(f'{zero_count = }, {one_count = }')
    print(f'{gamma_raw = }, {epsilon_raw = }')

    gamma_rate = int(gamma_raw, 2)
    epsilon_rate = int(epsilon_raw, 2)
    print(f'{gamma_rate = }, {epsilon_rate = }')

    power_consumption = gamma_rate * epsilon_rate
    print(f'{power_consumption = }')
