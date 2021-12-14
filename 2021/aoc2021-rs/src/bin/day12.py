import numpy as np
import collections as cl
import networkx as nx
import copy

with open('../inputs/day12.txt') as f:
    data = f.read()


import heapq

g = []
for line in data.strip().split('\n'):
    src, dest = line.split('-')
    g.append((src, dest))


G = nx.Graph()
G.add_edges_from(g)


def is_valid_path(path):
    seen = set()
    for pt in path:
        # Is lower
        if pt.lower() == pt:
            if pt in seen:
                return False
            seen.add(pt)

    return True


def is_valid_path_2(path):
    counts = cl.Counter(path)
    
    if counts['start'] > 1 or counts['end'] > 1:
        return False
    
    del counts['start']
    del counts['end']
    already_visited_one_twice = False
    for pt, count in counts.items():
        # pt is uppercase
        if pt.upper() == pt:
            continue
        else:
            if count == 2:
                if already_visited_one_twice:
                    return False
                else:
                    already_visited_one_twice = True
            elif count == 1:
                continue
            else:
                return False
            
    return True

# BFS
def bfs(G, validator_fn):
    src = 'start'

    # Create a queue which stores
    # the paths
    q = cl.deque()

    # Path vector to store the current path
    path = []
    path.append(src)
    q.append(path.copy())

    dst = 'end'
    output = []
    g = G
    while q:
        path = q.popleft()
        last = path[len(path) - 1]

        # If last vertex is the desired destination
        # then print the path
        if (last == dst):
            output.append(path.copy())
            continue

        # Traverse to all the nodes connected to
        # current vertex and push new path to queue
        if (validator_fn(path)):
            for n in nx.neighbors(G, last):        
                newpath = path.copy()
                newpath.append(n)
                q.append(newpath)

    return output


part1 = bfs(G, is_valid_path)
print(len(part1))

part2 = bfs(G, is_valid_path_2)
print(len(part2))
