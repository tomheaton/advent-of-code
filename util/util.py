import requests


# TODO: fix login issue (selenium?)
def get_inputs(day: int) -> str:
    r = requests.get(f'https://adventofcode.com/2021/day/{day}/input')
    return r.text
