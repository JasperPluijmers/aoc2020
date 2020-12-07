import re

with open('input.txt') as f:
    input = [re.findall(r'(\d \w+ \w+ bag)|(^\w+ \w+ bag)', line) for line in f.readlines()]


class Bag:
    def __init__(self, name: str):
        self.name = name
        self.contains = {}
        self.contained_in = {}


bags = {}

for line in input:
    current_bag_name = line[0][1]
    if current_bag_name in bags:
        current_bag = bags[current_bag_name]
    else:
        current_bag = Bag(current_bag_name)
        bags[current_bag_name] = current_bag
    for contained_bag in line[1:]:
        amount, name = int(contained_bag[0].split(" ")[0]), re.sub(r"\d+ ", "", contained_bag[0])
        if name not in bags:
            bags[name] = Bag(name)
        current_bag.contains[name] = (amount, bags[name])
        bags[name].contained_in[current_bag.name] = (amount, bags[current_bag.name])


def calculate_contains(bag: Bag, names: set):
    names.add(bag.name)
    if len(bag.contained_in) == 0:
        return names
    else:
        for amount, contained_in_bag in bag.contained_in.values():
            names.update(calculate_contains(contained_in_bag, names))
        return names


def calculate_amount(bag: Bag, total: int, multiplier: int):
    if len(bag.contains) == 0:
        return total
    else:
        for amount, contained_in_bag in bag.contains.values():
            total += amount * multiplier
            total = calculate_amount(contained_in_bag, total,  multiplier * amount)
        return total


print(len(calculate_contains(bags["shiny gold bag"], set())))
print(calculate_amount(bags["shiny gold bag"], 0, 1))
