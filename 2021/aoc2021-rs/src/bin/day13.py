import re
import collections as cl

with open('../inputs/day13.txt') as f:
    data = f.read().strip()


points = []
folds = []
for line in data.split('\n'):
    if line.strip() == '':
        continue
    matches = re.match(r'(\d+),(\d+)$', line.strip())
    if matches:
        points.append((int(matches[1]), int(matches[2])))
    else:
        matches = re.match('fold along (x|y)=(\d+)', line)
        folds.append((matches[1], int(matches[2])))


paper = cl.defaultdict(bool)
for (x, y) in points:
    paper[(x, y)] = True

def compute_size(paper):
    paper = dict(((pos,val) for (pos,val) in paper.items() if val))
    min_x = min(paper.keys(), key=lambda pt: pt[0])[0]
    min_y = min(paper.keys(), key=lambda pt: pt[1])[1]
    max_x = max(paper.keys(), key=lambda pt: pt[0])[0]
    max_y = max(paper.keys(), key=lambda pt: pt[1])[1]

    width = max_x - min_x
    height = max_y - min_y

    return width, height

def do_fold(paper, fold):
    width, height = compute_size(paper)
    axis, fold_val = fold
    
    points = cl.defaultdict(bool)
    for (pt, val) in paper.items():
        if not val:
            continue
        x, y = pt

        if axis == 'x':
            if x > fold_val:
                new_x = 2*fold_val - x
            else:
                new_x = x
            new_pos = new_x, y
        elif axis == 'y':
            if y > fold_val:
                new_y = 2*fold_val - y
            else:
                new_y = y
            new_pos = x, new_y
        points[new_pos] = True
    
    width, height = compute_size(points)
    
    return points


def print_points(points):
    if isinstance(points, cl.defaultdict) or isinstance(points, dict):
        points = [i for i,val in points.items() if val]

    min_x = min(points, key=lambda pt: pt[0])[0]
    min_y = min(points, key=lambda pt: pt[1])[1]
    max_x = max(points, key=lambda pt: pt[0])[0]
    max_y = max(points, key=lambda pt: pt[1])[1]

    for y in range(min_y, max_y+1):
        for x in range(min_x, max_x+1):
            if (x, y) in points:
                print('#', end='')
            else:
                print('.', end='')
        print()


paper2 = do_fold(paper, folds[0])
print('Part 1:', len([i for i,val in paper2.items() if val]))

output = None
in_map = paper.copy()

for fold in folds:
    output = do_fold(in_map, fold)
    in_map = output

out_points = [i for i,val in output.items() if val]

print('Part 2:')
print_points(out_points)
