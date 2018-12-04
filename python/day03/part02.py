import sys
import re
import collections as cl
import itertools as it

pattern = re.compile(r'\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')


def parse_line(line):
    row = pattern.match(line)
    id = row[1]
    left = int(row[2])
    top = int(row[3])
    w = int(row[4])
    h = int(row[5])
    return id, left, top, w, h


def all_cells(left, top, w, h):
    return it.product(range(left, left+w), range(top, top+h))


claim_ids = {id: (left, top, w, h) for id, left, top, w, h in map(parse_line, sys.stdin)}
claim_counter = cl.Counter((x, y) for id, (left, top, w, h) in claim_ids.items()
                           for x, y in all_cells(left, top, w, h))

good_claim_id = next(id for id, (left, top, w, h) in claim_ids.items()
                     if all(claim_counter[(x, y)] == 1 for x, y in all_cells(left, top, w, h)))

print('Good claim id', good_claim_id)
