import string

fields = ['byr:', 'iyr:', 'eyr:', 'hgt:', 'hcl:', 'ecl:', 'pid:']
fields_bare = {'byr': lambda value: four_digit_valid(value, 1920, 2002),
               'iyr': lambda value: four_digit_valid(value, 2010, 2020),
               'eyr': lambda value: four_digit_valid(value, 2020, 2030),
               'hgt': lambda value: hgt_valid(value),
               'hcl': lambda value: hcl_valid(value),
               'ecl': lambda value: ecl_valid(value),
               'pid': lambda value: pid_valid(value),
               'cid': lambda value: True}


def pid_valid(value: str) -> bool:
    try:
        number = int(value)
        return len(value) == 9
    except:
        return False

valid_hcl = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
def ecl_valid(value: str) -> bool:
    return value in valid_hcl

def hcl_valid(value: str) -> bool:
    if (value[0] == '#') and (len(value) == 7):
        for char in value[1:]:
            if char not in string.digits + string.ascii_lowercase:
                return False
            return True
    return False

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
        if not fields_bare[field](value):
            return False
    return True

valids = 0

for input in inputs:
    if is_line_valid(input):
        valids +=1
print(valids)