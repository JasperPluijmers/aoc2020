with open('input.txt') as f:
    inputs = [line.replace("F", "0").replace("L", "0").replace("B", "1").replace("R", "1").strip() for line in f.readlines()]
ids = [(int(line[:7], base=2) * 8 + int(line[7:], base=2)) for line in inputs]
ids = sorted(ids)

print(max(ids))
old_id = min(ids) - 1
for id in ids:
    if id - old_id == 2:
        print(id)
    old_id = id