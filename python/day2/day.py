def split_the_thing(thing):
    thing = thing.strip().split('-')
    min = int(thing[0])
    thing = thing[1].split(' ')
    max = int(thing[0])
    letter = thing[1][0]
    word = thing[2]
    return min, max, letter, word


def correct(min, max, letter, word):
    return word.count(letter) >= min and word.count(letter) <= max


def correct2(first, second, letter, word):
    return (word[first - 1] == letter) != (word[second - 1] == letter)


with open('input.txt') as file:
    data = [split_the_thing(point) for point in file.readlines()]


print([correct(*point) for point in data].count(True))
print([correct2(*point) for point in data].count(True))