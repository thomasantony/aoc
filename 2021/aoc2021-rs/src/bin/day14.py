## Day 14 - Extended Polymerization
##
## Part 1
## Brute force it by taking pairs out of original string
## 
## Part 2
## Brute force may not work ...

import re
import collections as cl
import itertools as it
from functools import lru_cache

with open('../inputs/day14.txt') as f:
    data = f.read().strip()

lines = data.split('\n')

starter = lines[0]
assert lines[1] == ''

recipes = {}
for line in lines[2:]:
    pair, insert = line.split(' -> ')
    recipes[tuple(pair)] = insert

def pairwise(iterable):
    # pairwise('ABCDEFG') --> AB BC CD DE EF FG
    a, b = it.tee(iterable)
    next(b, None)
    return zip(a, b)

# def process(start, recipes):

@lru_cache
def poly_step(starter):
    global recipes
    if len(starter) < 2:
        return starter

    a, b = starter[:2]
    if (a, b) in recipes:
        insert = recipes[(a, b)]
        retval = (a, insert, b)
    else:
        retval = (a, b)
    
    return retval + poly_step(starter[1:])
        

def polymerize(starter, recipes):
    item = list(starter)
    import functools

    item_out = []
    for a, b in pairwise(starter):
        if (a, b) in recipes:
            insert = recipes[(a, b)]
            output = (a, insert, b)
        else:
            output = (a, b)
        item_out.append(output)

    output = []
    for i in item_out:
        if len(i) == 3:
            output.append(i[0])
            output.append(i[1])
        else:
            output.append(i[0])
    output.append(item_out[-1][-1])
    return output

item = tuple(starter)
for i in range(10):
    item = polymerize(item, recipes)

ctr = cl.Counter(item)
max_val = max(ctr.items(), key=lambda item: item[1])
min_val = min(ctr.items(), key=lambda item: item[1])

print('Part 1:', max_val[1]- min_val[1])

item = tuple(starter)
for i in range(40):
    print(i)
    item = polymerize(item, recipes)
    

ctr = cl.Counter(item)
max_val = max(ctr.items(), key=lambda item: item[1])
min_val = min(ctr.items(), key=lambda item: item[1])

print('Part 2:', max_val[1]- min_val[1])
