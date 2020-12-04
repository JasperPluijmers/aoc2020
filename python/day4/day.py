import re

fields_validities = {'byr': lambda value: four_digit_valid(value, 1920, 2002),
                   'iyr': lambda value: four_digit_valid(value, 2010, 2020),
                   'eyr': lambda value: four_digit_valid(value, 2020, 2030),
                   'hgt': lambda value: hgt_valid(value),
                   'hcl': lambda value: re.match(r'^#[\da-z]{6}$', value),
                   'ecl': lambda value: value in valid_hcl,
                   'pid': lambda value: re.match(r'^\d{9}$', value)}

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


def is_line_valid_weak(input_line: str) -> bool:
    entries = {entry.split(':')[0]: entry.split(':')[1] for entry in input_line.strip().split(" ")}
    for field in fields_validities.keys():
        if field not in entries:
            return False
    return True


def is_line_valid(input_line: str) -> bool:
    entries = {entry.split(':')[0]: entry.split(':')[1] for entry in input_line.strip().split(" ")}
    for field, validity in fields_validities.items():
        if not ((field in entries) and (validity(entries[field]))):
            return False
    return True


valids_weak = 0
valids = 0
for line in inputs:
    if is_line_valid_weak(line):
        valids_weak += 1
    if is_line_valid(line):
        valids += 1
print(valids_weak)
print(valids)
