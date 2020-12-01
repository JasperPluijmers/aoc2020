with open('input.txt') as file:
    data = [int(point) for point in file.readlines()]

low_data = [point for point in data if point < 1010]
high_data = [point for point in data if point > 1010]

for low_point in low_data:
    for high_point in high_data:
        if low_point + high_point == 2020:
                print(low_point)
                print(high_point)
                print(low_point * high_point)

for low_point in low_data:
    for low_point_2 in low_data:
        for high_point in high_data + low_data:
            if low_point + high_point + low_point_2 == 2020:
                print(low_point)
                print(high_point)
                print(low_point_2)
                print(low_point * high_point * low_point_2)