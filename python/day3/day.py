import functools
with open('input_gerjan.txt') as file:
    data = [[*line.strip()] for line in file.readlines()]


def calculate_trees(map, slop_x, slope_y):
    x = 0
    y = 0
    trees = 0
    while True:
        y = (y + slope_y)
        x = (x + slop_x) % len(map[0])
        if y > (len(map) - 1):
            return trees
        if map[y][x] == '#':
            trees += 1


slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
print(calculate_trees(data, 3, 1))
print([calculate_trees(data, *slope) for slope in slopes])
print(functools.reduce(lambda x, y: x*y, [calculate_trees(data, *slope) for slope in slopes]))
