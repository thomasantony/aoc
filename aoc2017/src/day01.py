from itertools import pairwise

with open('inputs/day01.txt') as f:
    data = f.read().strip()

data = data + data[0]

print(sum(int(a) for (a,b) in pairwise(data) if a == b))
