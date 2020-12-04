import re

fields = ['byr:', 'iyr:', 'eyr:', 'hgt:', 'hcl:', 'ecl:', 'pid:']
fields_validity = {'byr': lambda value: four_digit_valid(value, 1920, 2002),
               'iyr': lambda value: four_digit_valid(value, 2010, 2020),
               'eyr': lambda value: four_digit_valid(value, 2020, 2030),
               'hgt': lambda value: hgt_valid(value),
               'hcl': lambda value: re.match(r'^#[\da-z]{6}$', value),
               'ecl': lambda value: value in valid_hcl,
               'pid': lambda value: re.match(r'^\d{9}$', value),
               'cid': lambda value: True}


valid_hcl = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}


def hgt_valid(value: str) -> bool:
    unit = value[-2:]
    try:
        number = int(value[:-2])
    except:
        return False
    if unit == 'cm':
        return (number >= 150) & (number <= 193)
    if unit == 'in':
        return (number >= 59) & (number <= 76)
    return False


def four_digit_valid(value: str, lower: int, upper: int) -> bool:
    try:
        digits = int(value)
    except:
        return False
    return (digits >= lower) and (digits <= upper)


with open('input.txt') as f:
    inputs = [line.replace("\n", " ") for line in f.read().split("\n\n")]


def is_line_valid(input_line: str):
    for field in fields:
        if field not in input_line:
            return False
    entry_list = input_line.strip().split(" ")
    for entry in entry_list:
        field, value = entry.split(":")
        if not fields_validity[field](value):
            return False
    return True

valids = 0

for input in inputs:
    if is_line_valid(input):
        valids +=1
print(valids)