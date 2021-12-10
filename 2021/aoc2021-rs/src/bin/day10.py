# Day 10 - Syntax Scoring
# Use stack to figure out the correct order of open/close tags
import collections as cl

with open('../inputs/day10.txt') as f:
    data = f.read()

ctr = cl.Counter()

open_tags = {'(': ')', '[': ']', '{': '}', '<': '>'}

score = 0
scores = {')': 3, ']': 57, '}': 1197, '>': 25137}

valid_lines = []
for i, line in enumerate(data.splitlines()):
    # print(i)
    stack = []
    is_invalid = False
    for c in line.strip():
        if c in open_tags:
            stack.append(c)
            continue
        elif c in open_tags.values():
            o = stack.pop()
            if open_tags[o] == c:
                continue
            else:
                score += scores[c]
                is_invalid = True
                break
    if not is_invalid:
        valid_lines.append(line.strip())

print('Part 1:', score)

p2_scores = {'(': 1, '[': 2, '{': 3, '<': 4}
scores = []
for i, line in enumerate(valid_lines):
    stack = []
    
    for c in line:
        if c in open_tags:
            stack.append(c)
            continue
        elif c in open_tags.values():
            o = stack.pop()
            if open_tags[o] != c:
                break
    line_score = 0
    while len(stack) > 0:
        c = stack.pop()
        s = p2_scores[c]
        line_score = line_score * 5 + s
    scores.append(line_score)

scores.sort()
print('Part 2:', scores[len(scores)//2])
