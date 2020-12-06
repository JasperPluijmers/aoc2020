from collections import Counter
with open('input.txt') as f:
    inputs = [line.strip().split("\n") for line in f.read().split("\n\n")]

any_yes = 0
all_yes = 0
for form in inputs:
    any_yes += len(set("".join(form)))
    all_yes += len([question for question, amount in Counter("".join(form)).items() if amount == len(form)])
print(any_yes)
print(all_yes)
