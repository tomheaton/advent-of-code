# Day Nine - Part One


if __name__ == "__main__":

    with open("input.txt", "r") as input_file:

        for line in [x.strip("\n") for x in input_file.readlines()]:
            print(f'{line = }')
