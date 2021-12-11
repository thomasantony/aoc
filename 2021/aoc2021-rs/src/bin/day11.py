import numpy as np
import collections as cl

with open('../inputs/day11.txt') as f:
    data = f.read()

a = []
for line in data.strip().split('\n'):
    b = list(map(int, list(line)))
    a.append(b)

input = np.array(a)


neigh = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]]


def do_flash(pos, levels, alredy_flashed):
    flashes = 0
    i, j = pos
    if levels[i,j] > 9:
        levels[i,j] = 0
        alredy_flashed.append((i, j))
        for (di, dj) in neigh:
            n_i, n_j = i+di, j+dj
            if n_i < 0 or n_i >= levels.shape[0] or n_j < 0 or n_j >= levels.shape[1]:
                continue
            
            levels[n_i, n_j] += 1
            if (n_i, n_j) not in alredy_flashed:
                flashes += do_flash((n_i, n_j), levels, alredy_flashed)
    return flashes

def increment_neigh(pos, levels):
    i, j = pos
    
    neigh_pos = []
    for (di, dj) in neigh:
        n_i, n_j = i+di, j+dj
        if n_i < 0 or n_i >= levels.shape[0] or n_j < 0 or n_j >= levels.shape[1]:
            continue
        
        levels[n_i, n_j] += 1
        neigh_pos.append((n_i, n_j))
    
    return neigh_pos


def step(levels):
    # flashes = 0

    for i in range(levels.shape[0]):
        for j in range(levels.shape[1]):
            levels[i, j] += 1
    

    alredy_flashed = set()
    last_levels = None
    any_flashed = True
    while any_flashed:
        # if last_levels is not None and np.allclose(levels, last_levels):
        #     break
        any_flashed = False
        last_levels = np.copy(levels)
        for i in range(levels.shape[0]):
            for j in range(levels.shape[1]):
                # do flash if level >= 9
                if levels[i, j] > 9 and (i, j) not in alredy_flashed: 
                    # flashes += 1
                    alredy_flashed.add((i, j))
                    any_flashed = True
                    increment_neigh((i, j), levels)

    for (i, j) in list(alredy_flashed):
        levels[i, j] = 0
    return len(list(alredy_flashed))

levels = np.copy(input)

total = 0

for i in range(100):
    total += step(levels)
    # print(levels)

print(total)

i = 0

levels = np.copy(input)
z = np.zeros_like(levels)
while True:
    i+= 1
    step(levels)
    if np.allclose(levels, z):
        break

print(i)
